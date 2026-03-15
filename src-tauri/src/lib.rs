//! Smart Drug Inventory — Tauri application entry point
//!
//! Registers all modules and exposes Tauri IPC commands to the Vue frontend.

mod commands;
mod database;
mod kpi;
mod queries;
mod settings;

use commands::{
    get_drug_kpi_detail, get_drug_kpi_list, get_kpi_summary, get_settings, get_warehouses,
    health_check, save_settings, test_db_connection,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|_app| {
            // Initialise the database connection pool in the background.
            // We use `tauri::async_runtime::spawn` so that startup is not blocked
            // if the DB is unreachable — the user can fix settings via the UI.
            tauri::async_runtime::spawn(async {
                match database::init_pool().await {
                    Ok(()) => {
                        log::info!("Database pool initialised successfully");
                        match database::test_connection().await {
                            Ok(version) => log::info!("Connected to SQL Server: {version}"),
                            Err(e) => log::warn!("DB test query failed (pool ok): {e}"),
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "Database pool init failed: {e} — \
                             user can configure via Settings UI"
                        );
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Settings
            get_settings,
            save_settings,
            test_db_connection,
            // Health
            health_check,
            // Warehouses
            get_warehouses,
            // KPI
            get_kpi_summary,
            get_drug_kpi_list,
            get_drug_kpi_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
