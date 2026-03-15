// api/index.ts — Tauri IPC wrappers for backend commands

import { invoke } from '@tauri-apps/api/core'
import type {
  AppSettings,
  DbConfig,
  DrugKpi,
  DrugKpiSummary,
  HealthResult,
  Warehouse,
  WarehouseKpi,
} from '@/types'

export async function healthCheck(): Promise<HealthResult> {
  return invoke<HealthResult>('health_check')
}

export async function getWarehouses(): Promise<Warehouse[]> {
  return invoke<Warehouse[]>('get_warehouses')
}

export async function getKpiSummary(
  stockId: string,
  year: number,
  monthFrom: number,
  monthTo: number,
  rollingMonths?: number,
  expiryDays?: number,
): Promise<WarehouseKpi> {
  return invoke<WarehouseKpi>('get_kpi_summary', {
    stockId,
    year,
    monthFrom,
    monthTo,
    rollingMonths: rollingMonths ?? null,
    expiryDays: expiryDays ?? null,
  })
}

export async function getDrugList(
  stockId: string,
  year: number,
  monthFrom: number,
  monthTo: number,
  filters: {
    rollingMonths?: number
    expiryDays?: number
    dosStatus?: string
    deadStockOnly?: boolean
    expiryOnly?: boolean
    nlem?: string
  } = {},
): Promise<DrugKpiSummary[]> {
  return invoke<DrugKpiSummary[]>('get_drug_kpi_list', {
    stockId,
    year,
    monthFrom,
    monthTo,
    rollingMonths: filters.rollingMonths ?? null,
    expiryDays: filters.expiryDays ?? null,
    dosStatus: filters.dosStatus ?? null,
    deadStockOnly: filters.deadStockOnly ?? null,
    expiryOnly: filters.expiryOnly ?? null,
    nlem: filters.nlem ?? null,
  })
}

export async function getDrugDetail(
  workingCode: string,
  stockId: string,
  year: number,
  monthFrom: number,
  monthTo: number,
  rollingMonths?: number,
  expiryDays?: number,
): Promise<DrugKpi> {
  return invoke<DrugKpi>('get_drug_kpi_detail', {
    workingCode,
    stockId,
    year,
    monthFrom,
    monthTo,
    rollingMonths: rollingMonths ?? null,
    expiryDays: expiryDays ?? null,
  })
}

// Settings commands
export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings')
}

export async function saveSettings(newSettings: AppSettings): Promise<void> {
  return invoke<void>('save_settings', { newSettings })
}

export async function testDbConnection(db: DbConfig): Promise<string> {
  return invoke<string>('test_db_connection', { db })
}
