<template>
    <div class="drug-table-wrap">
        <div class="table-toolbar">
            <div class="search-box">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="11" cy="11" r="8" />
                    <path d="m21 21-4.35-4.35" />
                </svg>
                <input v-model="search" placeholder="ค้นหาชื่อยา / รหัสยา..." class="search-input" />
            </div>
            <div class="table-meta mono"><span>{{ filtered.length.toLocaleString('th-TH') }} รายการ</span></div>
        </div>

        <div class="table-scroll">
            <table class="drug-table">
                <thead>
                    <tr>
                        <th @click="doSort('drug_name')" class="sortable">ชื่อยา</th>
                        <th>บัญชี</th>
                        <th @click="doSort('rm_qty')" class="sortable num">คงเหลือ</th>
                        <th @click="doSort('dos')" class="sortable num">DOS (วัน)</th>
                        <th class="num">สถานะ</th>
                        <th @click="doSort('turnover_rate')" class="sortable num">Turnover</th>
                        <th @click="doSort('overall_score')" class="sortable num">Score</th>
                        <th class="num">Grade</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="drug in paginated" :key="drug.working_code" class="drug-row"
                        @click="$emit('select', drug)">
                        <td class="drug-name-cell">
                            <span class="drug-name">{{ drug.drug_name }}</span>
                            <span v-if="drug.is_dead_stock" class="dead-badge">DEAD</span>
                        </td>
                        <td><span class="nlem-badge">{{ drug.nlem || '—' }}</span></td>
                        <td class="num mono">{{ fmt(drug.rm_qty) }}</td>
                        <td class="num mono">
                            <span v-if="drug.dos !== null">{{ drug.dos }}</span>
                            <span v-else class="text-muted">∞</span>
                        </td>
                        <td class="num">
                            <span class="status-chip" :class="drug.dos_status ?? ''">{{ statusLabel(drug.dos_status)
                            }}</span>
                        </td>
                        <td class="num mono">{{ drug.turnover_rate !== null ? drug.turnover_rate?.toFixed(2) : '—' }}
                        </td>
                        <td class="num">
                            <div class="score-cell">
                                <div class="score-bar-bg">
                                    <div class="score-bar-fill"
                                        :style="{ width: drug.overall_score + '%', background: gradeColor(drug.grade) }" />
                                </div>
                                <span class="mono">{{ drug.overall_score }}</span>
                            </div>
                        </td>
                        <td class="num">
                            <span class="grade-badge" :class="'grade-' + drug.grade.toLowerCase()">{{ drug.grade
                            }}</span>
                        </td>
                    </tr>
                    <tr v-if="filtered.length === 0">
                        <td colspan="8" class="empty-row">ไม่พบรายการ</td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div v-if="totalPages > 1" class="pagination">
            <button @click="page = Math.max(1, page - 1)" :disabled="page === 1" class="pg-btn">‹</button>
            <span class="mono">{{ page }} / {{ totalPages }}</span>
            <button @click="page = Math.min(totalPages, page + 1)" :disabled="page === totalPages"
                class="pg-btn">›</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { DrugKpiSummary, Grade, DosStatus } from '@/types'

const props = defineProps<{ drugs: DrugKpiSummary[] }>()
defineEmits<{ select: [drug: DrugKpiSummary] }>()

const search = ref('')
const sortKey = ref<keyof DrugKpiSummary>('overall_score')
const sortAsc = ref(false)
const page = ref(1)
const PER_PAGE = 20

const filtered = computed(() => {
    let d = props.drugs
    if (search.value) {
        const q = search.value.toLowerCase()
        d = d.filter((x) => x.drug_name.toLowerCase().includes(q))
    }
    return [...d].sort((a, b) => {
        const av = (a[sortKey.value] as number) ?? -Infinity
        const bv = (b[sortKey.value] as number) ?? -Infinity
        return sortAsc.value ? (av > bv ? 1 : -1) : av < bv ? 1 : -1
    })
})

const totalPages = computed(() => Math.max(1, Math.ceil(filtered.value.length / PER_PAGE)))
const paginated = computed(() => {
    const s = (page.value - 1) * PER_PAGE
    return filtered.value.slice(s, s + PER_PAGE)
})

watch(filtered, () => { page.value = 1 })

function doSort(key: string) {
    const k = key as keyof DrugKpiSummary
    if (sortKey.value === k) sortAsc.value = !sortAsc.value
    else { sortKey.value = k; sortAsc.value = false }
}

const fmt = (n: number | null) => n?.toLocaleString('th-TH') ?? '—'

const statusLabel = (s: DosStatus | null): string => {
    const map: Record<DosStatus, string> = { stockout_risk: 'Stockout', low_stock: 'น้อย', normal: 'ปกติ', overstock: 'เกิน' }
    return s ? map[s] ?? '—' : '—'
}

const gradeColor = (g: Grade): string => {
    const map: Record<Grade, string> = { A: '#22c55e', B: '#84cc16', C: '#f59e0b', D: '#ef4444' }
    return map[g] ?? '#8b9ab0'
}
</script>

<style scoped>
.drug-table-wrap {
    display: flex;
    flex-direction: column;
}

.table-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    border-bottom: 1px solid var(--border-dim);
}

.search-box {
    display: flex;
    align-items: center;
    gap: 9px;
    color: var(--text-muted);
}

.search-input {
    background: none;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: var(--font-body);
    font-size: 14px;
    width: 240px;
}

.search-input::placeholder {
    color: var(--text-muted);
}

.table-meta {
    font-size: 13px;
    color: var(--text-muted);
}

.table-scroll {
    overflow-x: auto;
}

.drug-table {
    width: 100%;
    border-collapse: collapse;
}

.drug-table th {
    padding: 11px 18px;
    text-align: left;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.09em;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border-soft);
    white-space: nowrap;
    user-select: none;
}

.drug-table th.num {
    text-align: right;
}

.drug-table th.sortable {
    cursor: pointer;
}

.drug-table th.sortable:hover {
    color: var(--text-secondary);
}

.drug-row {
    cursor: pointer;
    transition: background var(--dur-fast);
}

.drug-row:hover td {
    background: var(--bg-hover);
}

.drug-row:hover td:first-child {
    border-left: 2px solid var(--accent);
    padding-left: 16px;
}

.drug-table td {
    padding: 13px 18px;
    font-size: 13.5px;
    border-bottom: 1px solid var(--border-dim);
    vertical-align: middle;
    border-left: 2px solid transparent;
    transition: background var(--dur-fast);
}

.drug-table td.num {
    text-align: right;
}

.drug-name-cell {
    display: flex;
    align-items: center;
    gap: 9px;
    max-width: 280px;
}

.drug-name {
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.dead-badge {
    font-family: var(--font-mono);
    font-size: 10px;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 4px;
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
    flex-shrink: 0;
    letter-spacing: 0.04em;
}

.nlem-badge {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-muted);
}

.status-chip {
    font-size: 11px;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 20px;
    white-space: nowrap;
}

.status-chip.stockout_risk {
    background: rgba(239, 68, 68, 0.1);
    color: var(--status-danger);
}

.status-chip.low_stock {
    background: rgba(245, 158, 11, 0.1);
    color: var(--status-warn);
}

.status-chip.normal {
    background: rgba(34, 197, 94, 0.1);
    color: var(--status-ok);
}

.status-chip.overstock {
    background: rgba(59, 130, 246, 0.1);
    color: var(--status-info);
}

.score-cell {
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: flex-end;
}

.score-bar-bg {
    width: 56px;
    height: 4px;
    background: var(--bg-overlay);
    border-radius: 2px;
}

.score-bar-fill {
    height: 100%;
    border-radius: 2px;
    transition: width 0.5s var(--ease);
}

.grade-badge {
    font-family: var(--font-display);
    font-size: 13px;
    font-weight: 700;
    width: 30px;
    height: 30px;
    border-radius: 7px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.grade-a {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
    border: 1px solid rgba(34, 197, 94, 0.3);
}

.grade-b {
    background: rgba(132, 204, 22, 0.15);
    color: #84cc16;
    border: 1px solid rgba(132, 204, 22, 0.3);
}

.grade-c {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
    border: 1px solid rgba(245, 158, 11, 0.3);
}

.grade-d {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
}

.empty-row {
    text-align: center;
    color: var(--text-muted);
    font-size: 14px;
    padding: 48px 40px;
}

.pagination {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 18px;
    padding: 18px;
    border-top: 1px solid var(--border-dim);
    font-family: var(--font-mono);
    font-size: 13px;
    color: var(--text-secondary);
}

.pg-btn {
    background: var(--bg-elevated);
    border: 1px solid var(--border-soft);
    color: var(--text-secondary);
    border-radius: 6px;
    width: 32px;
    height: 32px;
    cursor: pointer;
    display: grid;
    place-items: center;
    font-size: 16px;
    transition: all var(--dur-fast);
}

.pg-btn:hover:not(:disabled) {
    border-color: var(--accent);
    color: var(--accent);
}

.pg-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
}

.text-muted {
    color: var(--text-muted);
}
</style>
