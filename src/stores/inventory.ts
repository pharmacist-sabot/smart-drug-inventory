// stores/inventory.ts — Pinia store for inventory data
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getSettings, getKpiSummary, getDrugList } from '@/api'
import type { WarehouseKpi, DrugKpiSummary } from '@/types'

export const useInventoryStore = defineStore('inventory', () => {
  // State
  const stockId = ref('')
  const stockName = ref('คลังยาใหญ่')

  const selectedYear = ref(new Date().getFullYear() + 543)
  const selectedMonthFrom = ref(new Date().getMonth() + 1)
  const selectedMonthTo = ref(new Date().getMonth() + 1)

  const summary = ref<WarehouseKpi | null>(null)
  const drugs = ref<DrugKpiSummary[]>([])
  const activeFilter = ref('all')

  const loadingSummary = ref(false)
  const loadingDrugs = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const filteredDrugs = computed(() => {
    if (activeFilter.value === 'all') return drugs.value
    if (activeFilter.value === 'dead_stock')
      return drugs.value.filter((d) => d.is_dead_stock)
    return drugs.value.filter((d) => d.dos_status === activeFilter.value)
  })

  const periodLabel = computed(() => {
    const months = [
      'ม.ค.', 'ก.พ.', 'มี.ค.', 'เม.ย.', 'พ.ค.', 'มิ.ย.',
      'ก.ค.', 'ส.ค.', 'ก.ย.', 'ต.ค.', 'พ.ย.', 'ธ.ค.',
    ]
    const from = months[selectedMonthFrom.value - 1]
    const to = months[selectedMonthTo.value - 1]
    const yr = selectedYear.value
    return from === to ? `${from} ${yr}` : `${from} – ${to} ${yr}`
  })

  // Actions

  /**
   * Load the configured stock ID from settings.
   * Falls back to 'STOCK1' if settings haven't been saved yet.
   * Call this once on app mount before calling loadData().
   */
  async function initStock() {
    try {
      const settings = await getSettings()
      stockId.value = settings.default_stock_id || 'STOCK1'
    } catch {
      stockId.value = 'STOCK1'
    }
  }

  async function loadData() {
    if (!stockId.value) return
    error.value = null
    activeFilter.value = 'all'
    loadingSummary.value = true
    loadingDrugs.value = true

    try {
      const [sum, drugList] = await Promise.all([
        getKpiSummary(
          stockId.value,
          selectedYear.value,
          selectedMonthFrom.value,
          selectedMonthTo.value,
        ),
        getDrugList(
          stockId.value,
          selectedYear.value,
          selectedMonthFrom.value,
          selectedMonthTo.value,
        ),
      ])
      summary.value = sum
      drugs.value = drugList
      // Update display name from what the backend resolved
      if (sum.stock_name) stockName.value = sum.stock_name
    } catch (e) {
      error.value = 'ไม่สามารถโหลดข้อมูลได้ กรุณาตรวจสอบการเชื่อมต่อฐานข้อมูล'
      summary.value = null
      drugs.value = []
    } finally {
      loadingSummary.value = false
      loadingDrugs.value = false
    }
  }

  function clearError() {
    error.value = null
  }

  return {
    stockId,
    stockName,
    selectedYear,
    selectedMonthFrom,
    selectedMonthTo,
    summary,
    drugs,
    activeFilter,
    loadingSummary,
    loadingDrugs,
    error,
    filteredDrugs,
    periodLabel,
    initStock,
    loadData,
    clearError,
  }
})
