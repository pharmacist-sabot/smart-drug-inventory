//! settings.rs — Persistent application settings (JSON file in app data directory)
//!
//! Stores database connection parameters and app preferences.
//! Settings file location: `{app_data_dir}/settings.json`

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

/// Global mutable settings instance
static SETTINGS: Lazy<Mutex<AppSettings>> = Lazy::new(|| Mutex::new(AppSettings::load()));

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub server: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    /// Use Windows Authentication instead of SQL auth
    pub use_windows_auth: bool,
    /// Trust the server certificate (self-signed)
    pub trust_cert: bool,
    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            server: "localhost".to_string(),
            port: 1433,
            database: "INVS".to_string(),
            username: "sa".to_string(),
            password: String::new(),
            use_windows_auth: false,
            trust_cert: true,
            connect_timeout_secs: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub db: DbConfig,
    /// Default rolling months for DOS calculation
    pub default_rolling_months: i32,
    /// Default near-expiry warning days
    pub default_expiry_days: i32,
    /// DEPT_ID of the main drug warehouse (e.g. "STOCK1")
    #[serde(default = "default_stock_id")]
    pub default_stock_id: String,
}

fn default_stock_id() -> String {
    "STOCK1".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            db: DbConfig::default(),
            default_rolling_months: 3,
            default_expiry_days: 90,
            default_stock_id: default_stock_id(),
        }
    }
}

// ---------------------------------------------------------------------------
// File path helper
// ---------------------------------------------------------------------------

fn settings_path() -> PathBuf {
    let project =
        directories::ProjectDirs::from("com", "smartdrug", "inventory").unwrap_or_else(|| {
            // Fallback: store next to the executable
            let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
            // `ProjectDirs` expects a valid home dir; if unavailable we fake it.
            directories::ProjectDirs::from_path(exe.parent().unwrap_or(&exe).to_path_buf())
                .expect("cannot determine settings directory")
        });
    let dir = project.data_dir();
    if !dir.exists() {
        let _ = fs::create_dir_all(dir);
    }
    dir.join("settings.json")
}

// ---------------------------------------------------------------------------
// Load / Save
// ---------------------------------------------------------------------------

impl AppSettings {
    /// Load settings from disk; returns `Default` if the file doesn't exist or is invalid.
    pub fn load() -> Self {
        let path = settings_path();
        match fs::read_to_string(&path) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Persist the current settings to disk.
    pub fn save(&self) -> Result<(), String> {
        let path = settings_path();
        let json =
            serde_json::to_string_pretty(self).map_err(|e| format!("serialize error: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("write error: {e}"))?;
        log::info!("Settings saved to {}", path.display());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Public API (thread-safe)
// ---------------------------------------------------------------------------

/// Return a clone of the current settings.
pub fn get_settings() -> AppSettings {
    SETTINGS.lock().expect("settings lock poisoned").clone()
}

/// Replace current settings and persist to disk.
pub fn update_settings(new: AppSettings) -> Result<(), String> {
    let mut guard = SETTINGS.lock().expect("settings lock poisoned");
    new.save()?;
    *guard = new;
    Ok(())
}

/// Return only the database config.
pub fn get_db_config() -> DbConfig {
    get_settings().db
}
