//! database.rs — SQL Server connection pool via tiberius + bb8
//!
//! Provides a global async connection pool that all Tauri commands share.
//! Read-only: this system never writes data to INVS.

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::sync::atomic::{AtomicPtr, Ordering};
use tiberius::{AuthMethod, Config, EncryptionLevel};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::settings::{get_db_config, DbConfig};

// ---------------------------------------------------------------------------
// Global pool — we store a raw pointer to a leaked Box<Pool> so that we
// always have a &'static Pool reference. On reconnect we simply leak a new
// Pool and swap the pointer (the old one stays leaked — acceptable for a
// desktop app that reconnects at most a handful of times).
// ---------------------------------------------------------------------------

static POOL_PTR: AtomicPtr<Pool<ConnectionManager>> = AtomicPtr::new(std::ptr::null_mut());

/// Maximum connections kept in the pool.
const MAX_POOL_SIZE: u32 = 4;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Initialise (or re-initialise) the global connection pool from current
/// settings. Call this at app startup and whenever the user saves new DB
/// settings.
pub async fn init_pool() -> Result<(), String> {
    let pool = create_pool_from_settings().await?;
    let boxed = Box::new(pool);
    let ptr = Box::into_raw(boxed);
    // Swap in the new pool. We intentionally leak the old one (if any) because
    // there may still be outstanding PooledConnections referencing it.
    POOL_PTR.store(ptr, Ordering::Release);
    Ok(())
}

/// Re-create the pool (e.g. after settings change). Alias for `init_pool`.
pub async fn reconnect_pool() -> Result<(), String> {
    init_pool().await
}

/// Obtain a connection from the pool.
pub async fn get_conn() -> Result<bb8::PooledConnection<'static, ConnectionManager>, String> {
    let pool = get_pool()?;
    pool.get()
        .await
        .map_err(|e| format!("pool connection error: {e}"))
}

/// Quick connectivity test — returns the SQL Server version string.
pub async fn test_connection() -> Result<String, String> {
    let mut conn = get_conn().await?;
    let row = conn
        .simple_query("SELECT @@VERSION")
        .await
        .map_err(|e| format!("query error: {e}"))?
        .into_row()
        .await
        .map_err(|e| format!("fetch error: {e}"))?
        .ok_or_else(|| "no version row returned".to_string())?;

    let version: &str = row.get(0).unwrap_or("unknown");
    let first_line = version.lines().next().unwrap_or(version);
    Ok(first_line.to_string())
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Get a &'static reference to the current pool.
fn get_pool() -> Result<&'static Pool<ConnectionManager>, String> {
    let ptr = POOL_PTR.load(Ordering::Acquire);
    if ptr.is_null() {
        return Err(
            "database pool not initialised — please configure the connection in Settings"
                .to_string(),
        );
    }
    // SAFETY: the pointer was created via Box::into_raw and is never
    // deallocated, so it is always valid for 'static reads.
    Ok(unsafe { &*ptr })
}

async fn create_pool_from_settings() -> Result<Pool<ConnectionManager>, String> {
    let db = get_db_config();
    let mgr = build_connection_manager(&db);
    Pool::builder()
        .max_size(MAX_POOL_SIZE)
        .build(mgr)
        .await
        .map_err(|e| format!("pool build error: {e}"))
}

/// Build a tiberius `Config` from our `DbConfig` settings.
///
/// Note: tiberius 0.12 only exposes `AuthMethod::Integrated` when compiled
/// with the `integrated-auth-gssapi` tiberius feature (and only on platforms
/// that support it). Since we don't enable that feature, we always use SQL
/// Server authentication. The `use_windows_auth` flag is accepted but logs
/// a warning and falls back to SQL auth.
fn apply_config(config: &mut Config, db: &DbConfig) {
    config.host(&db.server);
    config.port(db.port);
    config.database(&db.database);

    if db.use_windows_auth {
        // Windows Integrated Auth requires the `integrated-auth-gssapi` crate
        // feature in tiberius. We don't enable it, so always fall back.
        log::warn!(
            "Windows Authentication requested but not available in this build. \
             Falling back to SQL Server authentication."
        );
        config.authentication(AuthMethod::sql_server(&db.username, &db.password));
    } else {
        config.authentication(AuthMethod::sql_server(&db.username, &db.password));
    }

    if db.trust_cert {
        config.encryption(EncryptionLevel::NotSupported);
        config.trust_cert();
    } else {
        config.encryption(EncryptionLevel::Required);
    }
}

fn build_connection_manager(db: &DbConfig) -> ConnectionManager {
    let mut config = Config::new();
    apply_config(&mut config, db);
    ConnectionManager::new(config)
}

// ---------------------------------------------------------------------------
// Standalone connection (used for the settings "Test Connection" button)
// ---------------------------------------------------------------------------

/// Try to connect with the supplied config (not the global pool) and return
/// the server version. Useful for the "Test Connection" button in settings UI.
pub async fn test_connection_with(db: &DbConfig) -> Result<String, String> {
    let mut config = Config::new();
    apply_config(&mut config, db);

    let tcp = TcpStream::connect(format!("{}:{}", db.server, db.port))
        .await
        .map_err(|e| format!("TCP connect error: {e}"))?;
    tcp.set_nodelay(true)
        .map_err(|e| format!("set_nodelay error: {e}"))?;

    let mut client = tiberius::Client::connect(config, tcp.compat_write())
        .await
        .map_err(|e| format!("TDS connect error: {e}"))?;

    let row = client
        .simple_query("SELECT @@VERSION")
        .await
        .map_err(|e| format!("query error: {e}"))?
        .into_row()
        .await
        .map_err(|e| format!("fetch error: {e}"))?
        .ok_or_else(|| "no version row returned".to_string())?;

    let version: &str = row.get(0).unwrap_or("unknown");
    let first_line = version.lines().next().unwrap_or(version);
    Ok(first_line.to_string())
}
