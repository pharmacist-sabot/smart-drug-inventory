<template>
    <div class="dashboard">
        <!-- ─── Topbar ─── -->
        <div class="topbar">
            <div class="topbar-inner">
                <div class="topbar-group topbar-warehouse">
                    <span class="tb-label">คลัง</span>
                    <span class="tb-warehouse-name">
                        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
                            <polyline points="9 22 9 12 15 12 15 22" />
                        </svg>
                        {{ store.stockName }}
                    </span>
                </div>

                <div class="topbar-group">
                    <label class="tb-label">ปี (พ.ศ.)</label>
                    <select v-model="store.selectedYear" class="tb-select tb-narrow">
                        <option v-for="y in yearOptions" :key="y" :value="y">{{ y }}</option>
                    </select>
                </div>

                <div class="topbar-group">
                    <label class="tb-label">เดือน</label>
                    <select v-model="store.selectedMonthFrom" class="tb-select tb-narrow">
                        <option v-for="(m, i) in thaiMonths" :key="i" :value="i + 1">{{ m }}</option>
                    </select>
                    <span class="tb-dash">–</span>
                    <select v-model="store.selectedMonthTo" class="tb-select tb-narrow">
                        <option v-for="(m, i) in thaiMonths" :key="i" :value="i + 1">{{ m }}</option>
                    </select>
                </div>

                <button class="tb-btn" @click="store.loadData()" :disabled="store.loadingSummary || store.loadingDrugs">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"
                        stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 12a9 9 0 1 1-6.22-8.56" />
                        <polyline points="21 3 21 9 15 9" />
                    </svg>
                    โหลดข้อมูล
                </button>
            </div>
        </div>

        <!-- ─── Error Banner ─── -->
        <div v-if="store.error" class="error-banner">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10" />
                <line x1="12" y1="8" x2="12" y2="12" />
                <line x1="12" y1="16" x2="12.01" y2="16" />
            </svg>
            <span>{{ store.error }}</span>
            <button class="error-dismiss" @click="store.clearError()">✕</button>
        </div>

        <!-- ─── Loading Skeleton ─── -->
        <div v-if="store.loadingSummary && !store.summary" class="skeleton-wrap">
            <div class="skel skel-hero" />
            <div class="skel-row">
                <div class="skel skel-card" v-for="n in 4" :key="n" />
            </div>
            <div class="skel skel-panel" />
        </div>

        <!-- ─── Main Content ─── -->
        <template v-else-if="store.summary">
            <!-- Hero Section -->
            <section class="hero fadeUp">
                <div class="hero-left">
                    <GradeRing :score="store.summary.avg_overall_score" :grade="store.summary.grade" :size="120"
                        :stroke-w="7" />
                </div>
                <div class="hero-center">
                    <h1 class="hero-title">{{ store.summary.stock_name }}</h1>
                    <p class="hero-period">{{ store.periodLabel }}</p>
                    <p class="hero-drug-count mono">{{ store.summary.total_drugs }} รายการยา</p>
                </div>
                <div class="hero-metrics">
                    <div class="metric-item">
                        <span class="metric-value mono">{{ store.summary.avg_dos !== null ?
                            Math.round(store.summary.avg_dos) : '—'
                        }}</span>
                        <span class="metric-label">Avg DOS</span>
                    </div>
                    <div class="metric-divider" />
                    <div class="metric-item">
                        <span class="metric-value mono">{{ store.summary.avg_turnover !== null ?
                            store.summary.avg_turnover.toFixed(1) : '—' }}</span>
                        <span class="metric-label">Avg Turnover</span>
                    </div>
                    <div class="metric-divider" />
                    <div class="metric-item">
                        <span class="metric-value mono">{{ store.summary.dead_stock_count }}</span>
                        <span class="metric-label">Dead Stock</span>
                    </div>
                    <div class="metric-divider" />
                    <div class="metric-item">
                        <span class="metric-value mono">{{ fmtValue(store.summary.dead_stock_value) }}</span>
                        <span class="metric-label">Dead Stock ฿</span>
                    </div>
                </div>
            </section>

            <!-- Stat Cards -->
            <section class="stat-grid fadeUp" style="animation-delay: .06s">
                <StatCard label="Stockout Risk" :value="store.summary.stockout_count" unit="รายการ" :icon="iconStockout"
                    variant="danger" :sub="`จากทั้งหมด ${store.summary.total_drugs} รายการ`" />
                <StatCard label="Low Stock" :value="store.summary.low_stock_count" unit="รายการ" :icon="iconLowStock"
                    variant="warn" :sub="`จากทั้งหมด ${store.summary.total_drugs} รายการ`" />
                <StatCard label="ปกติ / Normal" :value="store.summary.normal_count" unit="รายการ" :icon="iconNormal"
                    variant="ok" :sub="`จากทั้งหมด ${store.summary.total_drugs} รายการ`" />
                <StatCard label="Overstock" :value="store.summary.overstock_count" unit="รายการ" :icon="iconOverstock"
                    variant="info" :sub="`จากทั้งหมด ${store.summary.total_drugs} รายการ`" />
            </section>

            <!-- DOS Distribution -->
            <section class="panel fadeUp" style="animation-delay: .12s">
                <div class="panel-header">
                    <h2 class="panel-title">DOS Distribution</h2>
                </div>
                <div class="panel-body">
                    <div class="dos-track">
                        <div v-for="seg in dosSegments" :key="seg.key" class="dos-seg" :style="{
                            width: store.summary.total_drugs > 0
                                ? (seg.count / store.summary.total_drugs * 100) + '%'
                                : '0%',
                            background: seg.color,
                        }" :title="`${seg.label}: ${seg.count}`" />
                    </div>
                    <div class="dos-legend">
                        <div v-for="seg in dosSegments" :key="seg.key" class="dos-legend-item">
                            <span class="dos-dot" :style="{ background: seg.color }" />
                            <span class="dos-legend-label">{{ seg.label }}</span>
                            <span class="dos-legend-count mono">{{ seg.count }}</span>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Issues Grid -->
            <section class="issues-grid fadeUp" style="animation-delay: .18s">
                <!-- Top Stockout Risk -->
                <div class="panel">
                    <div class="panel-header">
                        <h2 class="panel-title">🔴 Top Stockout Risk</h2>
                    </div>
                    <div class="panel-body">
                        <div v-if="store.summary.top_stockout.length === 0" class="empty-state">ไม่มีรายการ</div>
                        <ul v-else class="issue-list">
                            <li v-for="d in store.summary.top_stockout" :key="d.working_code" class="issue-item"
                                @click="goDetail(d)">
                                <span class="issue-name">{{ d.drug_name }}</span>
                                <div class="issue-meta">
                                    <span class="issue-dos mono">DOS {{ d.dos ?? '∞' }}</span>
                                    <span class="issue-grade" :class="'g-' + d.grade.toLowerCase()">{{ d.grade }}</span>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>

                <!-- Top Overstock -->
                <div class="panel">
                    <div class="panel-header">
                        <h2 class="panel-title">🔵 Top Overstock</h2>
                    </div>
                    <div class="panel-body">
                        <div v-if="store.summary.top_overstock.length === 0" class="empty-state">ไม่มีรายการ</div>
                        <ul v-else class="issue-list">
                            <li v-for="d in store.summary.top_overstock" :key="d.working_code" class="issue-item"
                                @click="goDetail(d)">
                                <span class="issue-name">{{ d.drug_name }}</span>
                                <div class="issue-meta">
                                    <span class="issue-dos mono">DOS {{ d.dos ?? '∞' }}</span>
                                    <span class="issue-grade" :class="'g-' + d.grade.toLowerCase()">{{ d.grade }}</span>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>

                <!-- Top Dead Stock -->
                <div class="panel">
                    <div class="panel-header">
                        <h2 class="panel-title">💀 Top Dead Stock</h2>
                    </div>
                    <div class="panel-body">
                        <div v-if="store.summary.top_dead_stock.length === 0" class="empty-state">ไม่มีรายการ</div>
                        <ul v-else class="issue-list">
                            <li v-for="d in store.summary.top_dead_stock" :key="d.working_code" class="issue-item"
                                @click="goDetail(d)">
                                <span class="issue-name">{{ d.drug_name }}</span>
                                <div class="issue-meta">
                                    <span class="issue-val mono">{{ fmtValue(d.rm_value) }}฿</span>
                                    <span class="issue-grade" :class="'g-' + d.grade.toLowerCase()">{{ d.grade }}</span>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
            </section>

            <!-- Drug Table Panel -->
            <section class="panel fadeUp" style="animation-delay: .24s">
                <div class="panel-header">
                    <h2 class="panel-title">รายการยาทั้งหมด</h2>
                </div>
                <div class="panel-filter-bar">
                    <FilterBar v-model="store.activeFilter" :filters="filterOptions" />
                </div>
                <DrugTable :drugs="store.filteredDrugs" @select="goDetail" />
            </section>
        </template>

        <!-- ─── No Data State ─── -->
        <div v-else class="empty-state-full">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"
                stroke-linecap="round" stroke-linejoin="round" style="opacity:.3">
                <path
                    d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
                <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
                <line x1="12" y1="22.08" x2="12" y2="12" />
            </svg>
            <p>เลือกคลังและช่วงเวลา แล้วกด <strong>โหลดข้อมูล</strong></p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useInventoryStore } from '@/stores/inventory'
import GradeRing from '@/components/GradeRing.vue'
import StatCard from '@/components/StatCard.vue'
import DrugTable from '@/components/DrugTable.vue'
import FilterBar from '@/components/FilterBar.vue'
import type { FilterOption } from '@/components/FilterBar.vue'
import type { DrugKpiSummary } from '@/types'

const router = useRouter()
const store = useInventoryStore()

const thaiMonths = ['ม.ค.', 'ก.พ.', 'มี.ค.', 'เม.ย.', 'พ.ค.', 'มิ.ย.', 'ก.ค.', 'ส.ค.', 'ก.ย.', 'ต.ค.', 'พ.ย.', 'ธ.ค.']

const currentBe = new Date().getFullYear() + 543
const yearOptions = Array.from({ length: 5 }, (_, i) => currentBe - i)

// ─── Icons (SVG strings) ───

const iconStockout = `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.29 3.86 1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>`

const iconLowStock = `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>`

const iconNormal = `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>`

const iconOverstock = `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="7" width="20" height="14" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/></svg>`

// ─── Helpers ───

function goDetail(drug: DrugKpiSummary) {
    router.push({
        path: `/drug/${drug.working_code}`,
        query: {
            stock_id: store.stockId,
            year: String(store.selectedYear),
            mf: String(store.selectedMonthFrom),
            mt: String(store.selectedMonthTo),
        },
    })
}

function fmtValue(v: number | null | undefined): string {
    if (v == null) return '—'
    if (v >= 1_000_000) return (v / 1_000_000).toFixed(1) + 'M'
    if (v >= 1_000) return (v / 1_000).toFixed(1) + 'K'
    return Math.round(v).toLocaleString('th-TH')
}

// ─── Computed ───

const dosSegments = computed(() => {
    const s = store.summary
    if (!s) return []
    return [
        { key: 'stockout', label: 'Stockout Risk', count: s.stockout_count, color: '#ef4444' },
        { key: 'low', label: 'Low Stock', count: s.low_stock_count, color: '#f59e0b' },
        { key: 'normal', label: 'Normal', count: s.normal_count, color: '#22c55e' },
        { key: 'overstock', label: 'Overstock', count: s.overstock_count, color: '#3b82f6' },
    ]
})

const filterOptions = computed<FilterOption[]>(() => {
    const s = store.summary
    const total = store.drugs.length
    const deadCount = store.drugs.filter((d) => d.is_dead_stock).length
    return [
        { key: 'all', label: 'ทั้งหมด', count: total, cls: 'all' },
        { key: 'stockout_risk', label: 'Stockout', count: s?.stockout_count ?? 0, cls: 'danger' },
        { key: 'low_stock', label: 'Low Stock', count: s?.low_stock_count ?? 0, cls: 'warn' },
        { key: 'normal', label: 'ปกติ', count: s?.normal_count ?? 0, cls: 'ok' },
        { key: 'overstock', label: 'Overstock', count: s?.overstock_count ?? 0, cls: 'info' },
        { key: 'dead_stock', label: 'Dead Stock', count: deadCount, cls: 'dead' },
    ]
})

// ─── Lifecycle ───

onMounted(async () => {
    // initStock reads default_stock_id from saved settings (falls back to 'STOCK1')
    await store.initStock()
    await store.loadData()
})
</script>

<style scoped>
/* ─── Dashboard Root ─── */
.dashboard {
    padding: 0 0 64px;
}

/* ─── Topbar ─── */
.topbar {
    position: sticky;
    top: 0;
    z-index: 100;
    background: rgba(10, 12, 20, 0.78);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border-bottom: 1px solid var(--border-dim, rgba(255, 255, 255, 0.06));
}

.topbar-inner {
    max-width: 1320px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 14px 28px;
    flex-wrap: wrap;
}

.topbar-group {
    display: flex;
    align-items: center;
    gap: 6px;
}

.tb-label {
    font-size: 12px;
    color: var(--text-muted, #6b7a90);
    text-transform: uppercase;
    letter-spacing: 0.07em;
    font-weight: 600;
    white-space: nowrap;
}

.topbar-warehouse {
    gap: 8px;
}

.tb-warehouse-name {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: var(--bg-elevated, #141820);
    border: 1px solid var(--border-soft, rgba(255, 255, 255, 0.08));
    border-radius: 8px;
    color: var(--accent, #00d4b8);
    font-family: var(--font-display, inherit);
    font-size: 13px;
    font-weight: 600;
    padding: 6px 14px;
    white-space: nowrap;
    letter-spacing: 0.01em;
}

.tb-select {
    background: var(--bg-elevated, #141820);
    border: 1px solid var(--border-soft, rgba(255, 255, 255, 0.08));
    border-radius: 8px;
    color: var(--text-primary, #e4e8ef);
    font-family: var(--font-body, inherit);
    font-size: 14px;
    padding: 7px 30px 7px 12px;
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1l4 4 4-4' stroke='%236b7a90' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    cursor: pointer;
    transition: border-color 0.15s;
}

.tb-select:focus {
    outline: none;
    border-color: var(--accent, #6366f1);
}

.tb-narrow {
    width: 96px;
}

.tb-dash {
    color: var(--text-muted, #6b7a90);
    font-size: 14px;
}

.tb-btn {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 8px 20px;
    margin-left: auto;
    background: var(--accent, #6366f1);
    color: #fff;
    border: none;
    border-radius: 8px;
    font-family: var(--font-body, inherit);
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s, transform 0.1s;
    white-space: nowrap;
}

.tb-btn:hover {
    opacity: 0.88;
}

.tb-btn:active {
    transform: scale(0.97);
}

.tb-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

/* ─── Error Banner ─── */
.error-banner {
    display: flex;
    align-items: center;
    gap: 12px;
    max-width: 1320px;
    margin: 20px auto 0;
    padding: 14px 24px;
    background: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 10px;
    color: #f87171;
    font-size: 14px;
}

.error-dismiss {
    margin-left: auto;
    background: none;
    border: none;
    color: #f87171;
    cursor: pointer;
    font-size: 14px;
    opacity: 0.7;
    transition: opacity 0.15s;
}

.error-dismiss:hover {
    opacity: 1;
}

/* ─── Loading Skeleton ─── */
.skeleton-wrap {
    max-width: 1320px;
    margin: 0 auto;
    padding: 32px 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.skel {
    background: linear-gradient(90deg, rgba(255, 255, 255, 0.04) 25%, rgba(255, 255, 255, 0.08) 50%, rgba(255, 255, 255, 0.04) 75%);
    background-size: 400% 100%;
    border-radius: 12px;
    animation: shimmer 1.6s ease infinite;
}

.skel-hero {
    height: 140px;
}

.skel-row {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
}

.skel-card {
    height: 110px;
}

.skel-panel {
    height: 300px;
}

@keyframes shimmer {
    0% {
        background-position: 200% 0;
    }

    100% {
        background-position: -200% 0;
    }
}

/* ─── Hero Section ─── */
.hero {
    max-width: 1320px;
    margin: 32px auto 0;
    padding: 0 28px;
    display: flex;
    align-items: center;
    gap: 32px;
}

.hero-left {
    flex-shrink: 0;
}

.hero-center {
    flex: 1;
    min-width: 0;
    padding: 4px 0;
}

.hero-title {
    font-family: var(--font-display, inherit);
    font-size: 26px;
    font-weight: 700;
    color: var(--text-primary, #e4e8ef);
    margin: 0;
    line-height: 1.25;
}

.hero-period {
    font-size: 15px;
    color: var(--text-secondary, #8b9ab0);
    margin: 6px 0 0;
}

.hero-drug-count {
    font-size: 13px;
    color: var(--text-muted, #6b7a90);
    margin: 6px 0 0;
}

.hero-metrics {
    display: flex;
    align-items: center;
    gap: 28px;
    flex-shrink: 0;
}

.metric-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
}

.metric-value {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary, #e4e8ef);
    line-height: 1;
}

.metric-label {
    font-size: 11px;
    color: var(--text-muted, #6b7a90);
    text-transform: uppercase;
    letter-spacing: 0.07em;
    margin-top: 3px;
}

.metric-divider {
    width: 1px;
    height: 40px;
    background: var(--border-soft, rgba(255, 255, 255, 0.08));
}

/* ─── Stat Cards Grid ─── */
.stat-grid {
    max-width: 1320px;
    margin: 28px auto 0;
    padding: 0 28px;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 18px;
}

/* ─── Panel ─── */
.panel {
    max-width: 1320px;
    margin: 28px auto 0;
    background: var(--bg-elevated, #141820);
    border: 1px solid var(--border-soft, rgba(255, 255, 255, 0.08));
    border-radius: 14px;
    overflow: hidden;
}

/* Panels within grids get auto margin removed */
.issues-grid .panel,
.stat-grid .panel {
    margin: 0;
    max-width: none;
}

/* Standalone panels use same padding pattern as stat-grid / issues-grid */
section.panel {
    margin-left: 28px;
    margin-right: 28px;
    max-width: calc(1320px - 56px);
}

@media (min-width: 1404px) {
    section.panel {
        margin-left: auto;
        margin-right: auto;
        max-width: 1320px;
    }
}

.panel-header {
    padding: 18px 24px;
    border-bottom: 1px solid var(--border-dim, rgba(255, 255, 255, 0.04));
}

.panel-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary, #e4e8ef);
    margin: 0;
}

.panel-body {
    padding: 22px 24px;
}

.panel-filter-bar {
    padding: 16px 24px 0;
}

/* ─── DOS Distribution ─── */
.dos-track {
    display: flex;
    height: 22px;
    border-radius: 6px;
    overflow: hidden;
    gap: 2px;
}

.dos-seg {
    transition: width 0.6s cubic-bezier(0.16, 1, 0.3, 1);
    min-width: 2px;
    border-radius: 3px;
}

.dos-legend {
    display: flex;
    gap: 24px;
    margin-top: 16px;
    flex-wrap: wrap;
}

.dos-legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
}

.dos-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    flex-shrink: 0;
}

.dos-legend-label {
    font-size: 13px;
    color: var(--text-secondary, #8b9ab0);
}

.dos-legend-count {
    font-size: 13px;
    color: var(--text-muted, #6b7a90);
}

/* ─── Issues Grid ─── */
.issues-grid {
    max-width: 1320px;
    margin: 28px auto 0;
    padding: 0 28px;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 18px;
}

.issue-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.issue-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 11px 14px;
    border-radius: 8px;
    cursor: pointer;
    transition: background 0.15s;
}

.issue-item:hover {
    background: var(--bg-hover, rgba(255, 255, 255, 0.04));
}

.issue-name {
    font-size: 13px;
    color: var(--text-primary, #e4e8ef);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
}

.issue-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
}

.issue-dos,
.issue-val {
    font-size: 12px;
    color: var(--text-muted, #6b7a90);
}

.issue-grade {
    font-family: var(--font-display, inherit);
    font-size: 11px;
    font-weight: 700;
    width: 22px;
    height: 22px;
    border-radius: 5px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.g-a {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
}

.g-b {
    background: rgba(132, 204, 22, 0.15);
    color: #84cc16;
}

.g-c {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
}

.g-d {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
}

/* ─── Empty State ─── */
.empty-state {
    text-align: center;
    padding: 32px 16px;
    color: var(--text-muted, #6b7a90);
    font-size: 14px;
}

.empty-state-full {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 18px;
    padding: 120px 24px;
    text-align: center;
    color: var(--text-muted, #6b7a90);
    font-size: 15px;
}

.empty-state-full strong {
    color: var(--text-secondary, #8b9ab0);
}

/* ─── Fade Up Animation ─── */
.fadeUp {
    animation: fadeUp 0.45s cubic-bezier(0.16, 1, 0.3, 1) both;
}

@keyframes fadeUp {
    from {
        opacity: 0;
        transform: translateY(16px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* ─── Responsive: 1100px ─── */
@media (max-width: 1100px) {
    .hero-metrics {
        gap: 18px;
    }

    .metric-value {
        font-size: 20px;
    }

    .stat-grid {
        grid-template-columns: repeat(2, 1fr);
    }

    .issues-grid {
        grid-template-columns: repeat(2, 1fr);
    }

    .issues-grid>.panel:last-child {
        grid-column: span 2;
    }
}

/* ─── Responsive: 860px ─── */
@media (max-width: 860px) {
    .hero {
        flex-direction: column;
        align-items: flex-start;
        gap: 24px;
    }

    .hero-left {
        display: flex;
        align-items: center;
        gap: 20px;
    }

    .hero-metrics {
        width: 100%;
        justify-content: space-between;
    }

    .issues-grid {
        grid-template-columns: 1fr;
    }

    .issues-grid>.panel:last-child {
        grid-column: span 1;
    }

    .topbar-inner {
        gap: 10px;
    }
}

/* ─── Responsive: 640px ─── */
@media (max-width: 640px) {
    .topbar-inner {
        padding: 12px 16px;
        gap: 10px;
    }

    .tb-btn {
        width: 100%;
        justify-content: center;
    }

    .hero {
        padding: 0 16px;
    }

    .hero-metrics {
        flex-wrap: wrap;
        gap: 12px;
    }

    .metric-divider {
        display: none;
    }

    .metric-item {
        flex: 1;
        min-width: 60px;
    }

    .stat-grid {
        grid-template-columns: 1fr 1fr;
        padding: 0 16px;
        gap: 10px;
    }

    .issues-grid {
        padding: 0 16px;
    }

    section.panel {
        margin-left: 16px;
        margin-right: 16px;
        max-width: calc(100% - 32px);
    }

    .panel-body {
        padding: 14px;
    }

    .dos-legend {
        gap: 12px;
    }

    .hero-title {
        font-size: 20px;
    }
}
</style>
