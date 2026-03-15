//! commands.rs — Tauri IPC commands invoked from the Vue frontend
//!
//! Each `#[tauri::command]` function is callable from TypeScript via `invoke()`.
//! All database access is read-only.

use std::collections::HashMap;

use crate::database;
use crate::kpi::{
    calculate_drug_kpi, calculate_warehouse_kpi, DrugKpi, DrugKpiSummary, ExpiryStatus,
    WarehouseKpi,
};
use crate::queries::{
    fetch_drug_movements, fetch_last_cost, fetch_near_expiry, fetch_warehouses, period_label,
    safe_unit_cost, to_date_range, Warehouse,
};
use crate::settings::{self, AppSettings, DbConfig};

// ─────────────────────────────────────────────
// Settings commands
// ─────────────────────────────────────────────

/// Return the current app settings.
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    Ok(settings::get_settings())
}

/// Save new app settings and reconnect the database pool.
#[tauri::command]
pub async fn save_settings(new_settings: AppSettings) -> Result<(), String> {
    settings::update_settings(new_settings)?;
    // Reconnect pool with new DB config
    database::reconnect_pool().await?;
    Ok(())
}

/// Test a database connection with the given config (without saving).
#[tauri::command]
pub async fn test_db_connection(db: DbConfig) -> Result<String, String> {
    database::test_connection_with(&db).await
}

// ─────────────────────────────────────────────
// Health check
// ─────────────────────────────────────────────

#[derive(serde::Serialize)]
pub struct HealthResult {
    pub api: String,
    pub database: DatabaseHealth,
}

#[derive(serde::Serialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub server: Option<String>,
    pub detail: Option<String>,
}

#[tauri::command]
pub async fn health_check() -> Result<HealthResult, String> {
    match database::test_connection().await {
        Ok(version) => Ok(HealthResult {
            api: "ok".to_string(),
            database: DatabaseHealth {
                status: "ok".to_string(),
                server: Some(version),
                detail: None,
            },
        }),
        Err(e) => Ok(HealthResult {
            api: "ok".to_string(),
            database: DatabaseHealth {
                status: "error".to_string(),
                server: None,
                detail: Some(e),
            },
        }),
    }
}

// ─────────────────────────────────────────────
// Warehouse list
// ─────────────────────────────────────────────

#[tauri::command]
pub async fn get_warehouses() -> Result<Vec<Warehouse>, String> {
    let mut conn = database::get_conn().await?;
    fetch_warehouses(&mut conn).await
}

// ─────────────────────────────────────────────
// KPI Summary (warehouse level)
// ─────────────────────────────────────────────

#[derive(serde::Serialize)]
pub struct WarehouseKpiOut {
    #[serde(flatten)]
    pub inner: WarehouseKpi,
    pub rolling_months: i32,
}

#[tauri::command]
pub async fn get_kpi_summary(
    stock_id: String,
    year: i32,
    month_from: u32,
    month_to: u32,
    rolling_months: Option<i32>,
    expiry_days: Option<i32>,
) -> Result<WarehouseKpiOut, String> {
    let app_settings = settings::get_settings();
    let rolling = rolling_months.unwrap_or(app_settings.default_rolling_months);
    let exp_days = expiry_days.unwrap_or(app_settings.default_expiry_days);

    let (date_from, date_to) = to_date_range(year, month_from, month_to);
    let period = period_label(year, month_from, month_to);

    // Fetch warehouse name
    let mut conn = database::get_conn().await?;
    let warehouses = fetch_warehouses(&mut conn).await?;
    let stock_name = warehouses
        .iter()
        .find(|w| w.dept_id == stock_id)
        .map_or_else(|| stock_id.clone(), |w| w.dept_name.clone());
    drop(conn);

    // Fetch drug movements
    let mut conn = database::get_conn().await?;
    let movement_result =
        fetch_drug_movements(&mut conn, &stock_id, date_from, date_to, rolling).await?;
    drop(conn);

    // Fetch near-expiry lots
    let mut conn = database::get_conn().await?;
    let expiry_rows = fetch_near_expiry(&mut conn, &stock_id, date_to, exp_days).await?;
    drop(conn);

    // Build expiry map: working_code → lots
    let mut expiry_map: HashMap<String, Vec<crate::kpi::RawExpiryLotRow>> = HashMap::new();
    for lot in expiry_rows {
        expiry_map
            .entry(lot.WORKING_CODE.clone())
            .or_default()
            .push(lot);
    }

    // Calculate per-drug KPIs
    let mut drug_kpis: Vec<DrugKpi> = Vec::with_capacity(movement_result.rows.len());
    for row in &movement_result.rows {
        let unit_cost = if row.RM_QTY.unwrap_or(0.0) == 0.0 {
            let mut conn2 = database::get_conn().await?;
            let cost_data = fetch_last_cost(&mut conn2, &row.WORKING_CODE).await?;
            safe_unit_cost(cost_data)
        } else {
            0.0
        };

        let lots = expiry_map.get(&row.WORKING_CODE).map_or(&[][..], |v| v);
        drug_kpis.push(calculate_drug_kpi(
            row,
            movement_result.display_days,
            movement_result.rolling_days,
            unit_cost,
            lots,
        ));
    }

    let wh = calculate_warehouse_kpi(&drug_kpis, &stock_id, &stock_name, &period);

    Ok(WarehouseKpiOut {
        inner: wh,
        rolling_months: rolling,
    })
}

// ─────────────────────────────────────────────
// KPI Drug List
// ─────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn get_drug_kpi_list(
    stock_id: String,
    year: i32,
    month_from: u32,
    month_to: u32,
    rolling_months: Option<i32>,
    expiry_days: Option<i32>,
    // Filters
    dos_status: Option<String>,
    dead_stock_only: Option<bool>,
    expiry_only: Option<bool>,
    nlem: Option<String>,
) -> Result<Vec<DrugKpiSummary>, String> {
    let app_settings = settings::get_settings();
    let rolling = rolling_months.unwrap_or(app_settings.default_rolling_months);
    let exp_days = expiry_days.unwrap_or(app_settings.default_expiry_days);

    let (date_from, date_to) = to_date_range(year, month_from, month_to);

    let mut conn = database::get_conn().await?;
    let movement_result =
        fetch_drug_movements(&mut conn, &stock_id, date_from, date_to, rolling).await?;
    drop(conn);

    let mut conn = database::get_conn().await?;
    let expiry_rows = fetch_near_expiry(&mut conn, &stock_id, date_to, exp_days).await?;
    drop(conn);

    let mut expiry_map: HashMap<String, Vec<crate::kpi::RawExpiryLotRow>> = HashMap::new();
    for lot in expiry_rows {
        expiry_map
            .entry(lot.WORKING_CODE.clone())
            .or_default()
            .push(lot);
    }

    let mut drug_kpis: Vec<DrugKpi> = Vec::with_capacity(movement_result.rows.len());
    for row in &movement_result.rows {
        let unit_cost = if row.RM_QTY.unwrap_or(0.0) == 0.0 {
            let mut conn2 = database::get_conn().await?;
            let cost_data = fetch_last_cost(&mut conn2, &row.WORKING_CODE).await?;
            safe_unit_cost(cost_data)
        } else {
            0.0
        };

        let lots = expiry_map.get(&row.WORKING_CODE).map_or(&[][..], |v| v);
        drug_kpis.push(calculate_drug_kpi(
            row,
            movement_result.display_days,
            movement_result.rolling_days,
            unit_cost,
            lots,
        ));
    }

    // Apply filters
    let filtered: Vec<DrugKpiSummary> = drug_kpis
        .iter()
        .filter(|d| {
            if let Some(ref ds) = dos_status {
                if d.dos_status.label() != ds.as_str() {
                    return false;
                }
            }
            if dead_stock_only.unwrap_or(false) && !d.is_dead_stock {
                return false;
            }
            if expiry_only.unwrap_or(false)
                && !matches!(
                    d.expiry_status,
                    ExpiryStatus::Expired | ExpiryStatus::Critical | ExpiryStatus::Warning
                )
            {
                return false;
            }
            if let Some(ref n) = nlem {
                if d.nlem != *n {
                    return false;
                }
            }
            true
        })
        .map(super::kpi::DrugKpi::to_summary)
        .collect();

    Ok(filtered)
}

// ─────────────────────────────────────────────
// KPI Drug Detail
// ─────────────────────────────────────────────

#[tauri::command]
pub async fn get_drug_kpi_detail(
    working_code: String,
    stock_id: String,
    year: i32,
    month_from: u32,
    month_to: u32,
    rolling_months: Option<i32>,
    expiry_days: Option<i32>,
) -> Result<DrugKpi, String> {
    let app_settings = settings::get_settings();
    let rolling = rolling_months.unwrap_or(app_settings.default_rolling_months);
    let exp_days = expiry_days.unwrap_or(app_settings.default_expiry_days);

    let (date_from, date_to) = to_date_range(year, month_from, month_to);

    let mut conn = database::get_conn().await?;
    let movement_result =
        fetch_drug_movements(&mut conn, &stock_id, date_from, date_to, rolling).await?;
    drop(conn);

    let target = movement_result
        .rows
        .iter()
        .find(|r| r.WORKING_CODE == working_code)
        .ok_or_else(|| format!("ไม่พบยารหัส {working_code}"))?
        .clone();

    let unit_cost = if target.RM_QTY.unwrap_or(0.0) == 0.0 {
        let mut conn2 = database::get_conn().await?;
        let cost_data = fetch_last_cost(&mut conn2, &working_code).await?;
        safe_unit_cost(cost_data)
    } else {
        0.0
    };

    let mut conn = database::get_conn().await?;
    let expiry_rows = fetch_near_expiry(&mut conn, &stock_id, date_to, exp_days).await?;
    drop(conn);

    let lots: Vec<_> = expiry_rows
        .into_iter()
        .filter(|l| l.WORKING_CODE == working_code)
        .collect();

    let drug = calculate_drug_kpi(
        &target,
        movement_result.display_days,
        movement_result.rolling_days,
        unit_cost,
        &lots,
    );

    Ok(drug)
}
