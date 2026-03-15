// types/index.ts — Shared TypeScript types for the Smart Drug Inventory app

export type DosStatus = 'stockout_risk' | 'low_stock' | 'normal' | 'overstock'
export type ExpiryStatus = 'expired' | 'critical' | 'warning' | 'safe'
export type Grade = 'A' | 'B' | 'C' | 'D'

export interface DbConfig {
  server: string
  port: number
  database: string
  username: string
  password: string
  use_windows_auth: boolean
  trust_cert: boolean
  connect_timeout_secs: number
}

export interface AppSettings {
  db: DbConfig
  default_rolling_months: number
  default_expiry_days: number
  default_stock_id: string
}

export interface Warehouse {
  dept_id: string
  dept_name: string
}

export interface DatabaseHealth {
  status: string
  server?: string
  detail?: string
}

export interface HealthResult {
  api: string
  database: DatabaseHealth
}

export interface ExpiryLot {
  lot_no: string
  expired_date: number
  days_to_expire: number
  remain_qty_lot: number
  remain_value_lot: number
  status: ExpiryStatus
}

export interface DrugKpiSummary {
  working_code: string
  drug_name: string
  nlem: string
  rm_qty: number
  rm_value: number
  dis_qty: number
  rolling_dis_qty: number
  rolling_days: number
  dos: number | null
  dos_status: DosStatus | null
  turnover_rate: number | null
  is_dead_stock: boolean
  expiry_status: ExpiryStatus
  near_expiry_value: number
  overall_score: number
  grade: Grade
}

export interface DrugKpi {
  working_code: string
  drug_name: string
  nlem: string
  last_pack_ratio: number

  display_days: number
  rolling_days: number

  fw_qty: number
  rcv_qty: number
  dis_qty: number
  rm_qty: number
  rm_value: number
  dis_value: number
  qty_on_hand: number
  unit_cost: number

  rolling_dis_qty: number

  avg_stock: number | null
  daily_usage: number | null
  turnover_rate: number | null
  dos: number | null
  dos_status: DosStatus
  is_dead_stock: boolean
  dead_stock_value: number
  discrepancy: number

  expiry_status: ExpiryStatus
  expiry_lots: ExpiryLot[]
  near_expiry_qty: number
  near_expiry_value: number

  turnover_score: number
  dos_score: number
  dead_stock_score: number
  accuracy_score: number
  expiry_score: number
  overall_score: number
  grade: Grade
}

export interface WarehouseKpi {
  stock_id: string
  stock_name: string
  period: string
  rolling_months: number
  total_drugs: number

  stockout_count: number
  low_stock_count: number
  normal_count: number
  overstock_count: number

  dead_stock_count: number
  dead_stock_value: number

  expiry_expired_count: number
  expiry_critical_count: number
  expiry_warning_count: number
  near_expiry_value: number

  avg_turnover: number | null
  avg_dos: number | null
  avg_overall_score: number
  grade: Grade

  top_stockout: DrugKpiSummary[]
  top_overstock: DrugKpiSummary[]
  top_dead_stock: DrugKpiSummary[]
  top_expiry: DrugKpiSummary[]
}
