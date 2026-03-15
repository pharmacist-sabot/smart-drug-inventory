//! kpi.rs — KPI calculation logic
//!
//! Ported from the Python `kpi.py` module.
//!
//! Input : raw drug movement data from `queries::fetch_drug_movements()`
//! Output: `DrugKpi` / `WarehouseKpi` with scores and grades
//!
//! Rolling Window DOS:
//! ─────────────────────────────────────────────────────────────────
//! `calculate_drug_kpi()` receives:
//!   `rolling_dis_qty` : dispensed quantity in rolling window
//!   `rolling_days`    : actual days in rolling window
//!   `display_days`    : actual days in display period (used for Turnover)
//!
//! DOS = rm_qty / (rolling_dis_qty / rolling_days)
//!     → reflects average dispensing rate over multiple months
//!
//! Dead Stock = rm_qty > 0 AND rolling_dis_qty == 0 AND rolling_days >= 60
//!     → uses rolling window to confirm truly no dispensing
//! ─────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────

const DOS_STOCKOUT: f64 = 7.0;
const DOS_LOW: f64 = 15.0;
const DOS_NORMAL_HI: f64 = 60.0;
const DOS_OVER_MED: f64 = 90.0;
const DOS_OVER_HIGH: f64 = 120.0;

/// Dead stock: rolling window must be at least this many days
const DEAD_STOCK_MIN_DAYS: i32 = 60;

const EXPIRY_EXPIRED: i32 = 0;
const EXPIRY_CRITICAL: i32 = 30;
const EXPIRY_WARNING: i32 = 90;

const W_TURNOVER: f64 = 0.30;
const W_DOS: f64 = 0.25;
const W_DEAD_STOCK: f64 = 0.20;
const W_ACCURACY: f64 = 0.10;
const W_EXPIRY: f64 = 0.15;

// ─────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DosStatus {
    StockoutRisk,
    LowStock,
    Normal,
    Overstock,
}

impl DosStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::StockoutRisk => "stockout_risk",
            Self::LowStock => "low_stock",
            Self::Normal => "normal",
            Self::Overstock => "overstock",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryStatus {
    /// Worst — already expired
    Expired,
    Critical,
    Warning,
    /// Best — safe
    Safe,
}

impl ExpiryStatus {
    pub fn severity_order(self) -> u8 {
        match self {
            Self::Expired => 0,
            Self::Critical => 1,
            Self::Warning => 2,
            Self::Safe => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grade {
    A,
    B,
    C,
    D,
}

// ─────────────────────────────────────────────
// Data models
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiryLot {
    pub lot_no: String,
    pub expired_date: i64,
    pub days_to_expire: i32,
    pub remain_qty_lot: f64,
    pub remain_value_lot: f64,
    pub status: ExpiryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugKpi {
    // Identity
    pub working_code: String,
    pub drug_name: String,
    pub nlem: String,
    pub last_pack_ratio: f64,

    // Raw movement (display period)
    pub fw_qty: f64,
    pub rcv_qty: f64,
    /// Dispensed in display period (shown in movement summary)
    pub dis_qty: f64,
    pub rm_qty: f64,
    pub rm_value: f64,
    pub dis_value: f64,
    pub qty_on_hand: f64,
    pub unit_cost: f64,

    // Period context
    /// Actual days in display period (Turnover)
    pub display_days: i32,
    /// Actual days in rolling window (DOS, Dead Stock)
    pub rolling_days: i32,

    // Rolling values (for DOS / Dead Stock)
    /// Cumulative dispensed qty in rolling window
    pub rolling_dis_qty: f64,

    // Computed
    pub avg_stock: Option<f64>,
    /// rolling_dis_qty / rolling_days
    pub daily_usage: Option<f64>,
    /// Annualized from display period
    pub turnover_rate: Option<f64>,
    pub dos: Option<f64>,
    pub dos_status: DosStatus,
    pub is_dead_stock: bool,
    pub dead_stock_value: f64,
    pub discrepancy: f64,

    // Expiry
    pub expiry_status: ExpiryStatus,
    pub expiry_lots: Vec<ExpiryLot>,
    pub near_expiry_qty: f64,
    pub near_expiry_value: f64,

    // Scores
    pub turnover_score: f64,
    pub dos_score: f64,
    pub dead_stock_score: f64,
    pub accuracy_score: f64,
    pub expiry_score: f64,
    pub overall_score: f64,
    pub grade: Grade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugKpiSummary {
    pub working_code: String,
    pub drug_name: String,
    pub nlem: String,
    pub rm_qty: f64,
    pub rm_value: f64,
    pub dis_qty: f64,
    pub rolling_dis_qty: f64,
    pub rolling_days: i32,
    pub dos: Option<f64>,
    pub dos_status: Option<DosStatus>,
    pub turnover_rate: Option<f64>,
    pub is_dead_stock: bool,
    pub expiry_status: ExpiryStatus,
    pub near_expiry_value: f64,
    pub overall_score: f64,
    pub grade: Grade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseKpi {
    pub stock_id: String,
    pub stock_name: String,
    pub period: String,
    pub total_drugs: usize,

    pub stockout_count: usize,
    pub low_stock_count: usize,
    pub normal_count: usize,
    pub overstock_count: usize,

    pub dead_stock_count: usize,
    pub dead_stock_value: f64,

    pub expiry_expired_count: usize,
    pub expiry_critical_count: usize,
    pub expiry_warning_count: usize,
    pub near_expiry_value: f64,

    pub avg_turnover: Option<f64>,
    pub avg_dos: Option<f64>,
    pub avg_overall_score: f64,
    pub grade: Grade,

    pub top_stockout: Vec<DrugKpiSummary>,
    pub top_overstock: Vec<DrugKpiSummary>,
    pub top_dead_stock: Vec<DrugKpiSummary>,
    pub top_expiry: Vec<DrugKpiSummary>,
}

// ─────────────────────────────────────────────
// Pure calculation functions
// ─────────────────────────────────────────────

/// Daily dispensing rate calculated from the rolling window.
/// Returns `None` if there was no dispensing in the rolling window.
pub fn calc_daily_usage(rolling_dis_qty: f64, rolling_days: i32) -> Option<f64> {
    if rolling_dis_qty <= 0.0 || rolling_days <= 0 {
        return None;
    }
    Some(rolling_dis_qty / f64::from(rolling_days))
}

/// Days of Supply = rm_qty / daily_usage.
/// Returns `None` → dead stock (no dispensing in rolling window).
pub fn calc_dos(daily_usage: Option<f64>, rm_qty: f64) -> Option<f64> {
    match daily_usage {
        Some(du) if du > 0.0 => Some(round1(rm_qty / du)),
        _ => None,
    }
}

/// Annualized Turnover from the display period.
/// = (dis_qty / display_days * 365) / avg_stock
pub fn calc_turnover_annualized(
    fw_qty: f64,
    dis_qty: f64,
    rm_qty: f64,
    display_days: i32,
) -> Option<f64> {
    let avg_stock = (fw_qty + rm_qty) / 2.0;
    if avg_stock <= 0.0 || display_days <= 0 {
        return None;
    }
    Some(round4(
        (dis_qty / f64::from(display_days) * 365.0) / avg_stock,
    ))
}

pub fn get_dos_status(dos: Option<f64>) -> DosStatus {
    match dos {
        Some(d) if d < DOS_STOCKOUT => DosStatus::StockoutRisk,
        Some(d) if d < DOS_LOW => DosStatus::LowStock,
        Some(d) if d <= DOS_NORMAL_HI => DosStatus::Normal,
        None | Some(_) => DosStatus::Overstock,
    }
}

pub fn get_expiry_status(days_to_expire: i32) -> ExpiryStatus {
    if days_to_expire <= EXPIRY_EXPIRED {
        ExpiryStatus::Expired
    } else if days_to_expire <= EXPIRY_CRITICAL {
        ExpiryStatus::Critical
    } else if days_to_expire <= EXPIRY_WARNING {
        ExpiryStatus::Warning
    } else {
        ExpiryStatus::Safe
    }
}

// ─────────────────────────────────────────────
// Scoring functions
// ─────────────────────────────────────────────

pub fn score_turnover(turnover: Option<f64>) -> f64 {
    match turnover {
        None => 30.0,
        Some(t) if t >= 6.0 => 100.0,
        Some(t) if t >= 4.0 => 85.0,
        Some(t) if t >= 2.5 => 65.0,
        Some(t) if t >= 1.5 => 45.0,
        Some(t) if t >= 0.8 => 25.0,
        Some(_) => 10.0,
    }
}

pub fn score_dos(dos: Option<f64>) -> f64 {
    match dos {
        None => 0.0,
        Some(d) if d < DOS_STOCKOUT => 10.0,
        Some(d) if d < DOS_LOW => 50.0,
        Some(d) if d <= DOS_NORMAL_HI => 100.0,
        Some(d) if d <= DOS_OVER_MED => 60.0,
        Some(d) if d <= DOS_OVER_HIGH => 35.0,
        Some(_) => 10.0,
    }
}

pub fn score_dead_stock(is_dead: bool, dead_value: f64, rm_value: f64) -> f64 {
    if !is_dead {
        return 100.0;
    }
    if rm_value <= 0.0 {
        return 0.0;
    }
    let ratio = dead_value / rm_value;
    if ratio >= 0.50 {
        0.0
    } else if ratio >= 0.30 {
        20.0
    } else if ratio >= 0.10 {
        50.0
    } else {
        70.0
    }
}

pub fn score_accuracy(discrepancy: f64, rm_qty: f64) -> f64 {
    if rm_qty <= 0.0 {
        return if discrepancy == 0.0 { 100.0 } else { 50.0 };
    }
    let ratio = discrepancy.abs() / rm_qty;
    if ratio == 0.0 {
        100.0
    } else if ratio <= 0.01 {
        90.0
    } else if ratio <= 0.05 {
        70.0
    } else if ratio <= 0.10 {
        40.0
    } else {
        10.0
    }
}

pub fn score_expiry(lots: &[ExpiryLot]) -> f64 {
    if lots.is_empty() {
        return 100.0;
    }
    let has_expired = lots.iter().any(|l| l.status == ExpiryStatus::Expired);
    if has_expired {
        return 0.0;
    }
    let has_critical = lots.iter().any(|l| l.status == ExpiryStatus::Critical);
    if has_critical {
        return 20.0;
    }
    let has_warning = lots.iter().any(|l| l.status == ExpiryStatus::Warning);
    if has_warning {
        return 60.0;
    }
    100.0
}

pub fn get_grade(score: f64) -> Grade {
    if score >= 80.0 {
        Grade::A
    } else if score >= 60.0 {
        Grade::B
    } else if score >= 40.0 {
        Grade::C
    } else {
        Grade::D
    }
}

// ─────────────────────────────────────────────
// Raw movement row (from SQL)
// ─────────────────────────────────────────────

/// Represents a single row returned from the drug movements query.
/// Field names match the SQL column aliases.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RawDrugRow {
    pub WORKING_CODE: String,
    pub DRUG_NAME: Option<String>,
    pub NLEM: Option<String>,
    pub LAST_PACK_RATIO: Option<f64>,
    pub FW_QTY: Option<f64>,
    pub RCV_QTY: Option<f64>,
    pub DIS_QTY: Option<f64>,
    pub RM_QTY: Option<f64>,
    pub RM_VALUE: Option<f64>,
    pub DIS_VALUE: Option<f64>,
    pub QTY_ON_HAND: Option<f64>,
    pub ROLLING_DIS_QTY: Option<f64>,
}

/// Represents a near-expiry lot row from the SQL query.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RawExpiryLotRow {
    pub WORKING_CODE: String,
    pub LOT_NO: Option<String>,
    pub EXPIRED_DATE: Option<i64>,
    pub days_to_expire: Option<i32>,
    pub remain_qty_lot: Option<f64>,
    pub remain_value_lot: Option<f64>,
}

// ─────────────────────────────────────────────
// Main entry point — single drug
// ─────────────────────────────────────────────

/// Calculate full KPI for one drug.
///
/// # Arguments
/// * `raw`           — one row from `fetch_drug_movements()`
/// * `display_days`  — actual days in the display period (Turnover)
/// * `rolling_days`  — actual days in the rolling window (DOS, Dead Stock)
/// * `fallback_unit_cost` — fallback unit cost when rm_value is 0
/// * `expiry_lot_rows`    — lot rows filtered to this drug
#[allow(clippy::too_many_lines)]
pub fn calculate_drug_kpi(
    raw: &RawDrugRow,
    display_days: i32,
    rolling_days: i32,
    fallback_unit_cost: f64,
    expiry_lot_rows: &[RawExpiryLotRow],
) -> DrugKpi {
    let fw_qty = raw.FW_QTY.unwrap_or(0.0);
    let rcv_qty = raw.RCV_QTY.unwrap_or(0.0);
    let dis_qty = raw.DIS_QTY.unwrap_or(0.0);
    let rm_qty = raw.RM_QTY.unwrap_or(0.0);
    let rm_value = raw.RM_VALUE.unwrap_or(0.0);
    let dis_value = raw.DIS_VALUE.unwrap_or(0.0);
    let qty_on_hand = raw.QTY_ON_HAND.unwrap_or(0.0);
    let rolling_dis_qty = raw.ROLLING_DIS_QTY.unwrap_or(0.0);

    // Unit cost
    let resolved_unit_cost = if rm_qty > 0.0 && rm_value > 0.0 {
        rm_value / rm_qty
    } else {
        fallback_unit_cost
    };

    // ── DOS: using rolling window ──
    let avg_stock = (fw_qty + rm_qty) / 2.0;
    let daily_usage = calc_daily_usage(rolling_dis_qty, rolling_days);
    let dos = calc_dos(daily_usage, rm_qty);
    let dos_status = get_dos_status(dos);

    // ── Turnover: using display period ──
    let turnover = calc_turnover_annualized(fw_qty, dis_qty, rm_qty, display_days);

    // ── Dead Stock ──
    let is_dead = rm_qty > 0.0 && rolling_dis_qty == 0.0 && rolling_days >= DEAD_STOCK_MIN_DAYS;
    let dead_value = if is_dead {
        rm_qty * resolved_unit_cost
    } else {
        0.0
    };

    let discrepancy = qty_on_hand - rm_qty;

    // ── Expiry lots ──
    let mut processed_lots: Vec<ExpiryLot> = Vec::new();
    let mut near_expiry_qty = 0.0_f64;
    let mut near_expiry_value = 0.0_f64;

    for lot in expiry_lot_rows {
        let dte = lot.days_to_expire.unwrap_or(9999);
        let status = get_expiry_status(dte);
        let rql = lot.remain_qty_lot.unwrap_or(0.0);
        let rvl = lot.remain_value_lot.unwrap_or(0.0);

        let el = ExpiryLot {
            lot_no: lot.LOT_NO.clone().unwrap_or_default(),
            expired_date: lot.EXPIRED_DATE.unwrap_or(0),
            days_to_expire: dte,
            remain_qty_lot: rql,
            remain_value_lot: rvl,
            status,
        };

        if matches!(
            status,
            ExpiryStatus::Expired | ExpiryStatus::Critical | ExpiryStatus::Warning
        ) {
            near_expiry_qty += rql;
            near_expiry_value += rvl;
        }

        processed_lots.push(el);
    }

    let expiry_status = if processed_lots.is_empty() {
        ExpiryStatus::Safe
    } else {
        processed_lots
            .iter()
            .map(|l| l.status)
            .min_by_key(|s| s.severity_order())
            .unwrap_or(ExpiryStatus::Safe)
    };

    // ── Scores ──
    let t_score = score_turnover(turnover);
    let d_score = score_dos(dos);
    let ds_score = score_dead_stock(is_dead, dead_value, rm_value);
    let a_score = score_accuracy(discrepancy, rm_qty);
    let e_score = score_expiry(&processed_lots);

    let overall = t_score * W_TURNOVER
        + d_score * W_DOS
        + ds_score * W_DEAD_STOCK
        + a_score * W_ACCURACY
        + e_score * W_EXPIRY;

    DrugKpi {
        working_code: raw.WORKING_CODE.clone(),
        drug_name: raw.DRUG_NAME.clone().unwrap_or_default(),
        nlem: raw.NLEM.clone().unwrap_or_default(),
        last_pack_ratio: raw.LAST_PACK_RATIO.unwrap_or(1.0),
        fw_qty,
        rcv_qty,
        dis_qty,
        rm_qty,
        rm_value,
        dis_value,
        qty_on_hand,
        unit_cost: resolved_unit_cost,
        display_days,
        rolling_days,
        rolling_dis_qty,
        avg_stock: Some(round2(avg_stock)),
        daily_usage: daily_usage.map(round4),
        turnover_rate: turnover,
        dos,
        dos_status,
        is_dead_stock: is_dead,
        dead_stock_value: round2(dead_value),
        discrepancy: round2(discrepancy),
        expiry_status,
        expiry_lots: processed_lots,
        near_expiry_qty: round2(near_expiry_qty),
        near_expiry_value: round2(near_expiry_value),
        turnover_score: round1(t_score),
        dos_score: round1(d_score),
        dead_stock_score: round1(ds_score),
        accuracy_score: round1(a_score),
        expiry_score: round1(e_score),
        overall_score: round1(overall),
        grade: get_grade(overall),
    }
}

// ─────────────────────────────────────────────
// Conversion: DrugKpi → DrugKpiSummary
// ─────────────────────────────────────────────

impl DrugKpi {
    pub fn to_summary(&self) -> DrugKpiSummary {
        DrugKpiSummary {
            working_code: self.working_code.clone(),
            drug_name: self.drug_name.clone(),
            nlem: self.nlem.clone(),
            rm_qty: self.rm_qty,
            rm_value: self.rm_value,
            dis_qty: self.dis_qty,
            rolling_dis_qty: self.rolling_dis_qty,
            rolling_days: self.rolling_days,
            dos: self.dos,
            dos_status: Some(self.dos_status),
            turnover_rate: self.turnover_rate,
            is_dead_stock: self.is_dead_stock,
            expiry_status: self.expiry_status,
            near_expiry_value: self.near_expiry_value,
            overall_score: self.overall_score,
            grade: self.grade,
        }
    }
}

// ─────────────────────────────────────────────
// Warehouse-level KPI
// ─────────────────────────────────────────────

/// Aggregate KPI across all drugs in a warehouse.
#[allow(clippy::too_many_lines)]
pub fn calculate_warehouse_kpi(
    drug_kpis: &[DrugKpi],
    stock_id: &str,
    stock_name: &str,
    period: &str,
) -> WarehouseKpi {
    if drug_kpis.is_empty() {
        return WarehouseKpi {
            stock_id: stock_id.to_string(),
            stock_name: stock_name.to_string(),
            period: period.to_string(),
            total_drugs: 0,
            stockout_count: 0,
            low_stock_count: 0,
            normal_count: 0,
            overstock_count: 0,
            dead_stock_count: 0,
            dead_stock_value: 0.0,
            expiry_expired_count: 0,
            expiry_critical_count: 0,
            expiry_warning_count: 0,
            near_expiry_value: 0.0,
            avg_turnover: None,
            avg_dos: None,
            avg_overall_score: 0.0,
            grade: Grade::D,
            top_stockout: Vec::new(),
            top_overstock: Vec::new(),
            top_dead_stock: Vec::new(),
            top_expiry: Vec::new(),
        };
    }

    let total_drugs = drug_kpis.len();
    let mut stockout_count: usize = 0;
    let mut low_stock_count: usize = 0;
    let mut normal_count: usize = 0;
    let mut overstock_count: usize = 0;
    let mut dead_stock_count: usize = 0;
    let mut dead_stock_value = 0.0_f64;
    let mut expiry_expired_count: usize = 0;
    let mut expiry_critical_count: usize = 0;
    let mut expiry_warning_count: usize = 0;
    let mut near_expiry_value = 0.0_f64;

    let mut turnover_vals: Vec<f64> = Vec::new();
    let mut dos_vals: Vec<f64> = Vec::new();

    for d in drug_kpis {
        match d.dos_status {
            DosStatus::StockoutRisk => stockout_count += 1,
            DosStatus::LowStock => low_stock_count += 1,
            DosStatus::Normal => normal_count += 1,
            DosStatus::Overstock => overstock_count += 1,
        }

        if d.is_dead_stock {
            dead_stock_count += 1;
            dead_stock_value += d.dead_stock_value;
        }

        match d.expiry_status {
            ExpiryStatus::Expired => expiry_expired_count += 1,
            ExpiryStatus::Critical => expiry_critical_count += 1,
            ExpiryStatus::Warning => expiry_warning_count += 1,
            ExpiryStatus::Safe => {}
        }
        near_expiry_value += d.near_expiry_value;

        if let Some(t) = d.turnover_rate {
            turnover_vals.push(t);
        }
        if let Some(dos) = d.dos {
            dos_vals.push(dos);
        }
    }

    let avg_turnover = if turnover_vals.is_empty() {
        None
    } else {
        Some(round3(
            turnover_vals.iter().sum::<f64>() / turnover_vals.len() as f64,
        ))
    };

    let avg_dos = if dos_vals.is_empty() {
        None
    } else {
        Some(round1(dos_vals.iter().sum::<f64>() / dos_vals.len() as f64))
    };

    let avg_overall_score =
        round1(drug_kpis.iter().map(|d| d.overall_score).sum::<f64>() / drug_kpis.len() as f64);
    let grade = get_grade(avg_overall_score);

    // ── Top issues ──

    // Top stockout: sorted by dos ascending (lowest first)
    let mut stockout_drugs: Vec<&DrugKpi> = drug_kpis
        .iter()
        .filter(|d| d.dos_status == DosStatus::StockoutRisk)
        .collect();
    stockout_drugs.sort_by(|a, b| {
        a.dos
            .unwrap_or(0.0)
            .partial_cmp(&b.dos.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_stockout: Vec<DrugKpiSummary> = stockout_drugs
        .iter()
        .take(10)
        .map(|d| d.to_summary())
        .collect();

    // Top overstock: sorted by dos descending (highest first)
    let mut overstock_drugs: Vec<&DrugKpi> = drug_kpis
        .iter()
        .filter(|d| d.dos_status == DosStatus::Overstock && d.dos.is_some())
        .collect();
    overstock_drugs.sort_by(|a, b| {
        b.dos
            .unwrap_or(0.0)
            .partial_cmp(&a.dos.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_overstock: Vec<DrugKpiSummary> = overstock_drugs
        .iter()
        .take(10)
        .map(|d| d.to_summary())
        .collect();

    // Top dead stock: sorted by dead_stock_value descending
    let mut dead_drugs: Vec<&DrugKpi> = drug_kpis.iter().filter(|d| d.is_dead_stock).collect();
    dead_drugs.sort_by(|a, b| {
        b.dead_stock_value
            .partial_cmp(&a.dead_stock_value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_dead_stock: Vec<DrugKpiSummary> =
        dead_drugs.iter().take(10).map(|d| d.to_summary()).collect();

    // Top expiry: drugs with expired or critical status, sorted by near_expiry_value desc
    let mut expiry_drugs: Vec<&DrugKpi> = drug_kpis
        .iter()
        .filter(|d| {
            matches!(
                d.expiry_status,
                ExpiryStatus::Expired | ExpiryStatus::Critical
            )
        })
        .collect();
    expiry_drugs.sort_by(|a, b| {
        b.near_expiry_value
            .partial_cmp(&a.near_expiry_value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_expiry: Vec<DrugKpiSummary> = expiry_drugs
        .iter()
        .take(10)
        .map(|d| d.to_summary())
        .collect();

    WarehouseKpi {
        stock_id: stock_id.to_string(),
        stock_name: stock_name.to_string(),
        period: period.to_string(),
        total_drugs,
        stockout_count,
        low_stock_count,
        normal_count,
        overstock_count,
        dead_stock_count,
        dead_stock_value: round2(dead_stock_value),
        expiry_expired_count,
        expiry_critical_count,
        expiry_warning_count,
        near_expiry_value: round2(near_expiry_value),
        avg_turnover,
        avg_dos,
        avg_overall_score,
        grade,
        top_stockout,
        top_overstock,
        top_dead_stock,
        top_expiry,
    }
}

// ─────────────────────────────────────────────
// Rounding helpers
// ─────────────────────────────────────────────

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

fn round4(v: f64) -> f64 {
    (v * 10000.0).round() / 10000.0
}

// ─────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_daily_usage() {
        assert_eq!(calc_daily_usage(0.0, 90), None);
        assert_eq!(calc_daily_usage(-5.0, 90), None);
        assert_eq!(calc_daily_usage(90.0, 0), None);
        let du = calc_daily_usage(90.0, 30).unwrap();
        assert!((du - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_calc_dos() {
        assert_eq!(calc_dos(None, 100.0), None);
        assert_eq!(calc_dos(Some(0.0), 100.0), None);
        let dos = calc_dos(Some(5.0), 100.0).unwrap();
        assert!((dos - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_calc_turnover() {
        // avg_stock = (100 + 50) / 2 = 75
        // turnover = (60 / 30 * 365) / 75 = 730 / 75 = 9.7333
        let t = calc_turnover_annualized(100.0, 60.0, 50.0, 30).unwrap();
        assert!((t - 9.7333).abs() < 0.001);

        assert!(calc_turnover_annualized(0.0, 0.0, 0.0, 30).is_none());
    }

    #[test]
    fn test_dos_status() {
        assert_eq!(get_dos_status(None), DosStatus::Overstock);
        assert_eq!(get_dos_status(Some(3.0)), DosStatus::StockoutRisk);
        assert_eq!(get_dos_status(Some(10.0)), DosStatus::LowStock);
        assert_eq!(get_dos_status(Some(30.0)), DosStatus::Normal);
        assert_eq!(get_dos_status(Some(60.0)), DosStatus::Normal);
        assert_eq!(get_dos_status(Some(61.0)), DosStatus::Overstock);
    }

    #[test]
    fn test_expiry_status() {
        assert_eq!(get_expiry_status(-5), ExpiryStatus::Expired);
        assert_eq!(get_expiry_status(0), ExpiryStatus::Expired);
        assert_eq!(get_expiry_status(15), ExpiryStatus::Critical);
        assert_eq!(get_expiry_status(30), ExpiryStatus::Critical);
        assert_eq!(get_expiry_status(60), ExpiryStatus::Warning);
        assert_eq!(get_expiry_status(90), ExpiryStatus::Warning);
        assert_eq!(get_expiry_status(91), ExpiryStatus::Safe);
    }

    #[test]
    fn test_scores() {
        assert_eq!(score_turnover(Some(8.0)), 100.0);
        assert_eq!(score_turnover(None), 30.0);
        assert_eq!(score_turnover(Some(0.5)), 10.0);

        assert_eq!(score_dos(None), 0.0);
        assert_eq!(score_dos(Some(3.0)), 10.0);
        assert_eq!(score_dos(Some(30.0)), 100.0);
        assert_eq!(score_dos(Some(200.0)), 10.0);

        assert_eq!(score_dead_stock(false, 0.0, 0.0), 100.0);
        assert_eq!(score_dead_stock(true, 0.0, 0.0), 0.0);
        assert_eq!(score_dead_stock(true, 600.0, 1000.0), 0.0);

        assert_eq!(score_accuracy(0.0, 100.0), 100.0);
        assert_eq!(score_accuracy(0.5, 100.0), 90.0);
        assert_eq!(score_accuracy(20.0, 100.0), 10.0);
    }

    #[test]
    fn test_grade() {
        assert_eq!(get_grade(85.0), Grade::A);
        assert_eq!(get_grade(80.0), Grade::A);
        assert_eq!(get_grade(79.9), Grade::B);
        assert_eq!(get_grade(60.0), Grade::B);
        assert_eq!(get_grade(40.0), Grade::C);
        assert_eq!(get_grade(39.9), Grade::D);
    }

    #[test]
    fn test_calculate_drug_kpi_normal() {
        let raw = RawDrugRow {
            WORKING_CODE: "MED001".to_string(),
            DRUG_NAME: Some("Paracetamol 500mg".to_string()),
            NLEM: Some("ก".to_string()),
            LAST_PACK_RATIO: Some(1.0),
            FW_QTY: Some(100.0),
            RCV_QTY: Some(50.0),
            DIS_QTY: Some(80.0),
            RM_QTY: Some(70.0),
            RM_VALUE: Some(7000.0),
            DIS_VALUE: Some(8000.0),
            QTY_ON_HAND: Some(70.0),
            ROLLING_DIS_QTY: Some(240.0),
        };

        let kpi = calculate_drug_kpi(&raw, 31, 90, 0.0, &[]);

        assert_eq!(kpi.working_code, "MED001");
        assert_eq!(kpi.drug_name, "Paracetamol 500mg");
        assert!(!kpi.is_dead_stock);
        assert_eq!(kpi.discrepancy, 0.0);
        assert!(kpi.dos.is_some());
        assert!(kpi.turnover_rate.is_some());
        assert!(kpi.overall_score > 0.0);
    }

    #[test]
    fn test_calculate_drug_kpi_dead_stock() {
        let raw = RawDrugRow {
            WORKING_CODE: "MED002".to_string(),
            DRUG_NAME: Some("Dead Drug".to_string()),
            NLEM: None,
            LAST_PACK_RATIO: Some(1.0),
            FW_QTY: Some(50.0),
            RCV_QTY: Some(0.0),
            DIS_QTY: Some(0.0),
            RM_QTY: Some(50.0),
            RM_VALUE: Some(5000.0),
            DIS_VALUE: Some(0.0),
            QTY_ON_HAND: Some(50.0),
            ROLLING_DIS_QTY: Some(0.0),
        };

        let kpi = calculate_drug_kpi(&raw, 31, 90, 0.0, &[]);

        assert!(kpi.is_dead_stock);
        assert!(kpi.dead_stock_value > 0.0);
        assert_eq!(kpi.dos, None);
        assert_eq!(kpi.dos_status, DosStatus::Overstock);
    }

    #[test]
    fn test_calculate_drug_kpi_with_expiry() {
        let raw = RawDrugRow {
            WORKING_CODE: "MED003".to_string(),
            DRUG_NAME: Some("Expiring Drug".to_string()),
            NLEM: Some("ข".to_string()),
            LAST_PACK_RATIO: Some(1.0),
            FW_QTY: Some(100.0),
            RCV_QTY: Some(20.0),
            DIS_QTY: Some(60.0),
            RM_QTY: Some(60.0),
            RM_VALUE: Some(6000.0),
            DIS_VALUE: Some(6000.0),
            QTY_ON_HAND: Some(60.0),
            ROLLING_DIS_QTY: Some(180.0),
        };

        let lots = vec![
            RawExpiryLotRow {
                WORKING_CODE: "MED003".to_string(),
                LOT_NO: Some("LOT-A".to_string()),
                EXPIRED_DATE: Some(20250601),
                days_to_expire: Some(15),
                remain_qty_lot: Some(20.0),
                remain_value_lot: Some(2000.0),
            },
            RawExpiryLotRow {
                WORKING_CODE: "MED003".to_string(),
                LOT_NO: Some("LOT-B".to_string()),
                EXPIRED_DATE: Some(20260101),
                days_to_expire: Some(200),
                remain_qty_lot: Some(40.0),
                remain_value_lot: Some(4000.0),
            },
        ];

        let kpi = calculate_drug_kpi(&raw, 31, 90, 0.0, &lots);

        assert_eq!(kpi.expiry_lots.len(), 2);
        assert_eq!(kpi.expiry_lots[0].status, ExpiryStatus::Critical);
        assert_eq!(kpi.expiry_lots[1].status, ExpiryStatus::Safe);
        assert_eq!(kpi.expiry_status, ExpiryStatus::Critical);
        assert!((kpi.near_expiry_qty - 20.0).abs() < 0.01);
        assert!((kpi.near_expiry_value - 2000.0).abs() < 0.01);
    }

    #[test]
    fn test_warehouse_kpi_empty() {
        let wh = calculate_warehouse_kpi(&[], "PH01", "Test Warehouse", "ม.ค. 2568");
        assert_eq!(wh.total_drugs, 0);
        assert_eq!(wh.grade, Grade::D);
    }

    #[test]
    fn test_warehouse_kpi_aggregation() {
        let raw1 = RawDrugRow {
            WORKING_CODE: "D001".to_string(),
            DRUG_NAME: Some("Drug 1".to_string()),
            FW_QTY: Some(100.0),
            DIS_QTY: Some(80.0),
            RM_QTY: Some(70.0),
            RM_VALUE: Some(7000.0),
            ROLLING_DIS_QTY: Some(240.0),
            QTY_ON_HAND: Some(70.0),
            ..Default::default()
        };
        let raw2 = RawDrugRow {
            WORKING_CODE: "D002".to_string(),
            DRUG_NAME: Some("Drug 2".to_string()),
            FW_QTY: Some(50.0),
            DIS_QTY: Some(0.0),
            RM_QTY: Some(50.0),
            RM_VALUE: Some(5000.0),
            ROLLING_DIS_QTY: Some(0.0),
            QTY_ON_HAND: Some(50.0),
            ..Default::default()
        };

        let kpi1 = calculate_drug_kpi(&raw1, 31, 90, 0.0, &[]);
        let kpi2 = calculate_drug_kpi(&raw2, 31, 90, 0.0, &[]);

        let wh = calculate_warehouse_kpi(&[kpi1, kpi2], "PH01", "Pharmacy", "มี.ค. 2568");

        assert_eq!(wh.total_drugs, 2);
        assert_eq!(wh.dead_stock_count, 1);
        assert!(wh.avg_overall_score > 0.0);
    }

    #[test]
    fn test_kpi_weights_sum_to_one() {
        let sum = W_TURNOVER + W_DOS + W_DEAD_STOCK + W_ACCURACY + W_EXPIRY;
        assert!(
            (sum - 1.0).abs() < 1e-9,
            "KPI weights must sum to 1.0, got {sum}"
        );
    }

    #[test]
    fn test_rounding_helpers() {
        assert_eq!(round1(3.14159), 3.1);
        assert_eq!(round2(3.14159), 3.14);
        assert_eq!(round3(3.14159), 3.142);
        assert_eq!(round4(3.14159), 3.1416);
    }
}
