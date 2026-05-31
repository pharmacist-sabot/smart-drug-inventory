<template>
    <div class="drug-detail">
        <div class="page-container">

            <!-- ── Back bar ── -->
            <div class="back-bar">
                <button class="back-btn" @click="router.push('/')">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round">
                        <path d="m15 18-6-6 6-6" />
                    </svg>
                    <span>กลับหน้าหลัก</span>
                </button>
                <div class="back-bar-meta mono">
                    <span v-if="drug">{{ drug.working_code }}</span>
                </div>
            </div>

            <!-- ── Loading ── -->
            <div v-if="loading" class="loading-wrap">
                <div class="spinner" />
                <span class="loading-text">กำลังโหลดข้อมูล...</span>
            </div>

            <!-- ── Error ── -->
            <div v-else-if="error" class="error-wrap">
                <div class="error-icon">⚠</div>
                <p class="error-text">{{ error }}</p>
                <button class="retry-btn" @click="fetchData">ลองอีกครั้ง</button>
            </div>

            <!-- ── Main content ── -->
            <template v-else-if="drug">

                <!-- Drug Header -->
                <section class="drug-header">
                    <div class="header-info">
                        <div class="header-top-row">
                            <span class="working-code mono">{{ drug.working_code }}</span>
                            <div class="meta-chips">
                                <span v-if="drug.nlem" class="chip chip-nlem">{{ drug.nlem }}</span>
                                <span v-if="drug.last_pack_ratio && drug.last_pack_ratio !== 1" class="chip chip-pack">
                                    Pack {{ drug.last_pack_ratio }}
                                </span>
                                <span v-if="drug.is_dead_stock" class="chip chip-dead">DEAD STOCK</span>
                            </div>
                        </div>
                        <h1 class="drug-name">{{ drug.drug_name }}</h1>
                        <div class="header-sub-row">
                            <div class="sub-item">
                                <span class="sub-label">คงเหลือ</span>
                                <span class="sub-value mono">{{ fmt(drug.rm_qty) }} หน่วย</span>
                            </div>
                            <span class="sub-divider" />
                            <div class="sub-item">
                                <span class="sub-label">มูลค่าคงเหลือ</span>
                                <span class="sub-value mono">฿{{ fmtMoney(drug.rm_value) }}</span>
                            </div>
                            <span class="sub-divider" />
                            <div class="sub-item">
                                <span class="sub-label">ต้นทุน / หน่วย</span>
                                <span class="sub-value mono">฿{{ drug.unit_cost?.toFixed(2) ?? '—' }}</span>
                            </div>
                        </div>
                    </div>
                    <div class="header-grade">
                        <GradeRing :score="drug.overall_score" :grade="drug.grade" :size="120" :stroke-w="8" />
                    </div>
                </section>

                <!-- KPI Breakdown -->
                <section class="content-section">
                    <h2 class="section-title">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2">
                            <path d="M3 3v18h18" />
                            <path d="m19 9-5 5-4-4-3 3" />
                        </svg>
                        KPI Breakdown
                    </h2>
                    <div class="kpi-grid">

                        <!-- Inventory Turnover -->
                        <div class="kpi-card">
                            <div class="kpi-card-header">
                                <div class="kpi-icon turnover-icon">
                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
                                        <path d="M21 3v5h-5" />
                                    </svg>
                                </div>
                                <span class="kpi-card-title">Inventory Turnover</span>
                                <span class="score-badge"
                                    :style="{ background: scoreColor(drug.turnover_score) + '22', color: scoreColor(drug.turnover_score), borderColor: scoreColor(drug.turnover_score) + '55' }">
                                    {{ drug.turnover_score }}
                                </span>
                            </div>
                            <div class="kpi-card-body">
                                <div class="kpi-big-value mono">
                                    {{ drug.turnover_rate !== null ? drug.turnover_rate.toFixed(2) : '—' }}
                                    <span class="kpi-unit">รอบ</span>
                                </div>
                                <div class="kpi-formula mono">
                                    <span class="formula-part">{{ fmt(drug.dis_qty) }}</span>
                                    <span class="formula-op">÷</span>
                                    <span class="formula-part">{{ drug.avg_stock !== null ?
                                        drug.avg_stock.toFixed(1) : '—' }}</span>
                                </div>
                                <div class="kpi-formula-label">dis_qty ÷ avg_stock</div>
                                <div class="kpi-progress-wrap">
                                    <div class="kpi-progress-track">
                                        <div class="kpi-progress-fill"
                                            :style="{ width: Math.min(drug.turnover_score, 100) + '%', background: scoreColor(drug.turnover_score) }" />
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Days of Supply -->
                        <div class="kpi-card">
                            <div class="kpi-card-header">
                                <div class="kpi-icon dos-icon">
                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <rect x="3" y="4" width="18" height="18" rx="2" />
                                        <path d="M16 2v4" />
                                        <path d="M8 2v4" />
                                        <path d="M3 10h18" />
                                    </svg>
                                </div>
                                <span class="kpi-card-title">Days of Supply</span>
                                <span class="score-badge"
                                    :style="{ background: scoreColor(drug.dos_score) + '22', color: scoreColor(drug.dos_score), borderColor: scoreColor(drug.dos_score) + '55' }">
                                    {{ drug.dos_score }}
                                </span>
                            </div>
                            <div class="kpi-card-body">
                                <div class="kpi-big-value mono">
                                    {{ drug.dos !== null ? drug.dos : '∞' }}
                                    <span class="kpi-unit">วัน</span>
                                </div>
                                <div class="dos-bar-area">
                                    <DosBar :dos="drug.dos" :status="drug.dos_status" />
                                </div>
                                <div class="kpi-formula mono">
                                    <span class="formula-part">{{ fmt(drug.rm_qty) }}</span>
                                    <span class="formula-op">÷</span>
                                    <span class="formula-part">{{ dailyUsage }}</span>
                                </div>
                                <div class="kpi-formula-label">rm_qty ÷ daily usage</div>
                            </div>
                        </div>

                        <!-- Dead Stock -->
                        <div class="kpi-card" :class="{ 'kpi-card-alert': drug.is_dead_stock }">
                            <div class="kpi-card-header">
                                <div class="kpi-icon dead-icon">
                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M10 2h4" />
                                        <path d="M12 14v-4" />
                                        <path d="M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6" />
                                        <path d="M9 17H4v5" />
                                    </svg>
                                </div>
                                <span class="kpi-card-title">Dead Stock</span>
                                <span class="score-badge"
                                    :style="{ background: scoreColor(drug.dead_stock_score) + '22', color: scoreColor(drug.dead_stock_score), borderColor: scoreColor(drug.dead_stock_score) + '55' }">
                                    {{ drug.dead_stock_score }}
                                </span>
                            </div>
                            <div class="kpi-card-body">
                                <div class="dead-status-row">
                                    <span v-if="drug.is_dead_stock" class="dead-detected">
                                        <span class="dead-dot pulse" />
                                        ตรวจพบ Dead Stock
                                    </span>
                                    <span v-else class="dead-normal">
                                        <span class="normal-dot" />
                                        ปกติ
                                    </span>
                                </div>
                                <div v-if="drug.is_dead_stock" class="dead-value-row">
                                    <span class="dead-val-label">มูลค่า Dead Stock</span>
                                    <span class="dead-val mono">฿{{ fmtMoney(drug.dead_stock_value) }}</span>
                                </div>
                                <div v-else class="dead-value-row">
                                    <span class="dead-val-label">สถานะ</span>
                                    <span class="dead-val-ok">มีการเบิกจ่ายปกติ</span>
                                </div>
                            </div>
                        </div>

                        <!-- Stock Accuracy -->
                        <div class="kpi-card">
                            <div class="kpi-card-header">
                                <div class="kpi-icon accuracy-icon">
                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path
                                            d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z" />
                                        <path d="m9 12 2 2 4-4" />
                                    </svg>
                                </div>
                                <span class="kpi-card-title">Stock Accuracy</span>
                                <span class="score-badge"
                                    :style="{ background: scoreColor(drug.accuracy_score) + '22', color: scoreColor(drug.accuracy_score), borderColor: scoreColor(drug.accuracy_score) + '55' }">
                                    {{ drug.accuracy_score }}
                                </span>
                            </div>
                            <div class="kpi-card-body">
                                <div class="accuracy-main">
                                    <div class="accuracy-discrepancy mono"
                                        :class="{ 'has-diff': drug.discrepancy !== 0 }">
                                        {{ drug.discrepancy > 0 ? '+' : '' }}{{ fmt(drug.discrepancy) }}
                                        <span class="kpi-unit">หน่วย</span>
                                    </div>
                                </div>
                                <div class="accuracy-counts">
                                    <div class="accuracy-row">
                                        <span class="acc-label">ระบบ (System)</span>
                                        <span class="acc-value mono">{{ fmt(drug.qty_on_hand) }}</span>
                                    </div>
                                    <div class="accuracy-row">
                                        <span class="acc-label">คงเหลือจริง (Actual)</span>
                                        <span class="acc-value mono">{{ fmt(drug.rm_qty) }}</span>
                                    </div>
                                    <div class="accuracy-row diff-row">
                                        <span class="acc-label">ผลต่าง</span>
                                        <span class="acc-value mono" :class="{ 'text-danger': drug.discrepancy !== 0 }">
                                            {{ drug.discrepancy > 0 ? '+' : '' }}{{ fmt(drug.discrepancy) }}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>

                    </div>
                </section>

                <!-- Movement Summary -->
                <section class="content-section">
                    <h2 class="section-title">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2">
                            <path d="M5 12h14" />
                            <path d="m12 5 7 7-7 7" />
                        </svg>
                        Movement Summary
                    </h2>
                    <div class="flow-diagram">
                        <div class="flow-node">
                            <span class="flow-label">ยอดยกมา</span>
                            <span class="flow-value mono">{{ fmt(drug.fw_qty) }}</span>
                        </div>
                        <div class="flow-arrow">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                stroke-width="2" stroke-linecap="round">
                                <path d="M5 12h14" />
                                <path d="m12 5 7 7-7 7" />
                            </svg>
                        </div>
                        <div class="flow-node flow-in">
                            <span class="flow-label">+ รับเข้า</span>
                            <span class="flow-value mono positive">+{{ fmt(drug.rcv_qty) }}</span>
                        </div>
                        <div class="flow-arrow">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                stroke-width="2" stroke-linecap="round">
                                <path d="M5 12h14" />
                                <path d="m12 5 7 7-7 7" />
                            </svg>
                        </div>
                        <div class="flow-node flow-out">
                            <span class="flow-label">− จ่ายออก</span>
                            <span class="flow-value mono negative">−{{ fmt(drug.dis_qty) }}</span>
                        </div>
                        <div class="flow-arrow">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                stroke-width="2" stroke-linecap="round">
                                <path d="M5 12h14" />
                                <path d="m12 5 7 7-7 7" />
                            </svg>
                        </div>
                        <div class="flow-node flow-remain">
                            <span class="flow-label">= คงเหลือ</span>
                            <span class="flow-value mono">{{ fmt(drug.rm_qty) }}</span>
                        </div>
                    </div>
                    <div class="movement-extra">
                        <div class="movement-stat">
                            <span class="movement-stat-label">มูลค่าจ่ายออก</span>
                            <span class="movement-stat-value mono">฿{{ fmtMoney(drug.dis_value) }}</span>
                        </div>
                        <div class="movement-stat">
                            <span class="movement-stat-label">มูลค่าคงเหลือ</span>
                            <span class="movement-stat-value mono">฿{{ fmtMoney(drug.rm_value) }}</span>
                        </div>
                        <div class="movement-stat">
                            <span class="movement-stat-label">การใช้ต่อวัน</span>
                            <span class="movement-stat-value mono">{{ dailyUsage }} หน่วย</span>
                        </div>
                        <div class="movement-stat">
                            <span class="movement-stat-label">ช่วงเวลา</span>
                            <span class="movement-stat-value mono">
                                {{ drug.display_days }} วัน
                                <span class="movement-stat-sub">(rolling {{ drug.rolling_days }})</span>
                            </span>
                        </div>
                    </div>
                </section>

                <!-- Score Breakdown -->
                <section class="content-section">
                    <h2 class="section-title">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2">
                            <path d="M12 20V10" />
                            <path d="M18 20V4" />
                            <path d="M6 20v-4" />
                        </svg>
                        Score Breakdown
                    </h2>
                    <div class="score-table">
                        <div class="score-header-row">
                            <span class="score-col-name">KPI</span>
                            <span class="score-col-weight">Weight</span>
                            <span class="score-col-raw">Score</span>
                            <span class="score-col-bar">Progress</span>
                            <span class="score-col-weighted">Weighted</span>
                        </div>
                        <div v-for="row in scoreRows" :key="row.label" class="score-row">
                            <span class="score-col-name">{{ row.label }}</span>
                            <span class="score-col-weight mono">{{ (row.weight * 100).toFixed(0) }}%</span>
                            <span class="score-col-raw mono" :style="{ color: scoreColor(row.score) }">
                                {{ row.score }}
                            </span>
                            <span class="score-col-bar">
                                <div class="score-bar-track">
                                    <div class="score-bar-fill"
                                        :style="{ width: Math.min(row.score, 100) + '%', background: scoreColor(row.score) }" />
                                </div>
                            </span>
                            <span class="score-col-weighted mono">{{ row.weighted.toFixed(1) }}</span>
                        </div>
                        <div class="score-total-row">
                            <span class="score-col-name">Total</span>
                            <span class="score-col-weight mono">100%</span>
                            <span class="score-col-raw"></span>
                            <span class="score-col-bar">
                                <div class="score-bar-track total-track">
                                    <div class="score-bar-fill total-fill"
                                        :style="{ width: Math.min(drug.overall_score, 100) + '%', background: scoreColor(drug.overall_score) }" />
                                </div>
                            </span>
                            <span class="score-col-weighted mono total-score"
                                :style="{ color: scoreColor(drug.overall_score) }">
                                {{ drug.overall_score.toFixed(1) }}
                            </span>
                        </div>
                    </div>
                </section>

                <!-- Expiry Lots -->
                <section v-if="drug.expiry_lots && drug.expiry_lots.length > 0" class="content-section">
                    <h2 class="section-title">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2">
                            <path d="M12 9v4" />
                            <path d="M12 17h.01" />
                            <path
                                d="M10.29 3.86 1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                        </svg>
                        Expiry Lots
                        <span class="expiry-count mono">({{ drug.expiry_lots.length }} lots)</span>
                    </h2>
                    <div class="expiry-table">
                        <div class="expiry-header">
                            <span>Lot No.</span>
                            <span class="num">วันหมดอายุ</span>
                            <span class="num">เหลืออีก</span>
                            <span class="num">จำนวน</span>
                            <span class="num">มูลค่า</span>
                            <span class="num">สถานะ</span>
                        </div>
                        <div v-for="lot in drug.expiry_lots" :key="lot.lot_no" class="expiry-row">
                            <span class="mono lot-no">{{ lot.lot_no }}</span>
                            <span class="num mono">{{ formatExpDate(lot.expired_date) }}</span>
                            <span class="num mono" :class="expiryDaysClass(lot.days_to_expire)">
                                {{ lot.days_to_expire }} วัน
                            </span>
                            <span class="num mono">{{ fmt(lot.remain_qty_lot) }}</span>
                            <span class="num mono">฿{{ fmtMoney(lot.remain_value_lot) }}</span>
                            <span class="num">
                                <span class="expiry-status-chip" :class="lot.status">
                                    {{ expiryStatusLabel(lot.status) }}
                                </span>
                            </span>
                        </div>
                    </div>
                </section>

            </template>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { getDrugDetail } from '@/api';
import DosBar from '@/components/DosBar.vue';
import GradeRing from '@/components/GradeRing.vue';
import type { DrugKpi, ExpiryStatus } from '@/types';

const props = defineProps<{ code: string }>();

const router = useRouter();
const route = useRoute();

const drug = ref<DrugKpi | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);

// ── helpers ─────────────────────────────────────────────
function fmt(n: number | null | undefined): string {
  if (n === null || n === undefined) return '—';
  return n.toLocaleString('th-TH');
}

function fmtMoney(v: number | null | undefined): string {
  if (v === null || v === undefined) return '—';
  if (Math.abs(v) >= 1_000_000) return `${(v / 1_000_000).toFixed(2)}M`;
  if (Math.abs(v) >= 1_000) return `${(v / 1_000).toFixed(1)}K`;
  return v.toLocaleString('th-TH', { maximumFractionDigits: 2 });
}

const dailyUsage = computed(() => {
  if (!drug.value) return '0';
  const du = drug.value.daily_usage;
  if (du === null || du === undefined || du === 0) return '0';
  return du.toFixed(2);
});

const dosColor = computed(() => {
  if (!drug.value) return 'var(--color-body)';
  const map: Record<string, string> = {
    stockout_risk: 'var(--status-danger)',
    low_stock: 'var(--status-warn)',
    normal: 'var(--status-ok)',
    overstock: 'var(--status-info)',
  };
  return map[drug.value.dos_status] || 'var(--color-body)';
});

function scoreColor(s: number): string {
  if (s >= 80) return 'var(--status-ok)';
  if (s >= 60) return 'var(--status-warn)';
  if (s >= 40) return 'var(--status-warn)';
  return 'var(--status-danger)';
}

// ── score weights ────────────────────────────────────────
const WEIGHTS = [
  { key: 'turnover', label: 'Turnover', weight: 0.3 },
  { key: 'dos', label: 'Days of Supply', weight: 0.25 },
  { key: 'dead_stock', label: 'Dead Stock', weight: 0.2 },
  { key: 'accuracy', label: 'Accuracy', weight: 0.1 },
  { key: 'expiry', label: 'Expiry', weight: 0.15 },
] as const;

const scoreRows = computed(() => {
  if (!drug.value) return [];
  const d = drug.value;
  const scoreMap: Record<string, number> = {
    turnover: d.turnover_score,
    dos: d.dos_score,
    dead_stock: d.dead_stock_score,
    accuracy: d.accuracy_score,
    expiry: d.expiry_score,
  };
  return WEIGHTS.map((w) => {
    const score = scoreMap[w.key] ?? 0;
    return { label: w.label, weight: w.weight, score, weighted: score * w.weight };
  });
});

// ── expiry helpers ───────────────────────────────────────
function formatExpDate(dateNum: number): string {
  const s = String(dateNum);
  if (s.length === 8) {
    return `${s.slice(6, 8)}/${s.slice(4, 6)}/${s.slice(0, 4)}`;
  }
  return String(dateNum);
}

function expiryDaysClass(days: number): string {
  if (days <= 0) return 'text-expired';
  if (days <= 30) return 'text-critical';
  if (days <= 90) return 'text-warning';
  return '';
}

function expiryStatusLabel(status: ExpiryStatus): string {
  const map: Record<ExpiryStatus, string> = {
    expired: 'หมดอายุ',
    critical: 'วิกฤต',
    warning: 'ใกล้หมด',
    safe: 'ปลอดภัย',
  };
  return map[status] ?? status;
}

// ── data fetching ────────────────────────────────────────
async function fetchData() {
  loading.value = true;
  error.value = null;
  const stockId = (route.query.stock_id as string) ?? '';
  const year = Number(route.query.year) || new Date().getFullYear();
  const mf = Number(route.query.mf) || 1;
  const mt = Number(route.query.mt) || 12;
  try {
    drug.value = await getDrugDetail(props.code, stockId, year, mf, mt);
  } catch (err: unknown) {
    error.value = err instanceof Error ? err.message : 'ไม่สามารถโหลดข้อมูลได้';
  } finally {
    loading.value = false;
  }
}

onMounted(fetchData);
</script>

<style scoped>
.drug-detail {
    min-height: 100vh;
    padding-bottom: 80px;
    color: var(--color-ink);
}
.page-container {
    max-width: 1100px;
    margin: 0 auto;
    padding: 0 32px;
}

.back-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 0 16px;
    border-bottom: 1px solid var(--color-hairline);
    margin-bottom: 4px;
}
.back-btn {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    background: none;
    border: 1px solid var(--color-hairline);
    color: var(--color-secondary);
}

.back-btn:hover {
    border-color: var(--color-accent-mint);
    color: var(--color-accent-mint);
    background: rgba(0, 212, 184, 0.06);
}
.back-bar-meta {
    font-size: 12px;
    color: var(--color-body);
    letter-spacing: 0.04em;
}

.loading-wrap {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 18px;
    padding: 140px 0;
}
.spinner-lg {
    --spinner-size: 40px;
    width: 40px;
    height: 40px;
    border: 3px solid var(--color-hairline);
    border-top-color: var(--color-accent-mint);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
}
.loading-text { font-size: 14px; color: var(--color-body); }

.error-wrap {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
    padding: 120px 0;
}
.error-icon { font-size: 36px; }
.error-text { font-size: 15px; color: var(--status-danger); }

.retry-btn {
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    color: var(--color-secondary);
    padding: 9px 22px;
    border-radius: var(--rounded-sm);
    cursor: pointer;
    font-family: var(--font-display);
    font-size: 14px;
    transition: all var(--dur-fast) var(--ease);
}
.retry-btn:hover {
    border-color: var(--color-accent-mint);
    color: var(--color-accent-mint);
}

.drug-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 32px;
    padding: 36px 0 32px;
    border-bottom: 1px solid var(--color-hairline);
}
.header-info { flex: 1; min-width: 0; }
.header-top-row {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
    margin-bottom: 14px;
}
.working-code {
    font-size: 12px;
    font-weight: var(--font-weight-semibold);
    color: var(--color-accent-mint);
    background: rgba(0, 212, 184, 0.08);
    padding: 4px 12px;
    border-radius: 5px;
    border: 1px solid rgba(0, 212, 184, 0.2);
    letter-spacing: 0.06em;
}
.meta-chips { display: flex; gap: 7px; flex-wrap: wrap; }
.chip {
    font-size: 11px;
    font-weight: var(--font-weight-bold);
    padding: 3px 10px;
    border-radius: 5px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
}
.chip-nlem {
    background: rgba(139, 92, 246, 0.12);
    color: #a78bfa;
    border: 1px solid rgba(139, 92, 246, 0.25);
}
.chip-pack {
    background: rgba(59, 130, 246, 0.1);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.2);
}
.chip-dead {
    background: rgba(239, 68, 68, 0.12);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
    animation: pulse-dead 2s ease-in-out infinite;
}

.drug-name {
    font-family: var(--font-display);
    font-size: 26px;
    font-weight: var(--font-weight-bold);
    color: var(--color-ink);
    margin: 0 0 16px;
    line-height: 1.3;
}
.header-sub-row {
    display: flex;
    align-items: center;
    gap: 0;
    flex-wrap: wrap;
}
.sub-item {
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding: 10px 20px;
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-md);
}
.sub-item+.sub-divider+.sub-item { border-left: none; border-radius: 0; }
.sub-divider {
    display: block;
    width: 1px;
    height: 44px;
    background: var(--color-hairline);
}
.header-sub-row .sub-item:first-child {
    border-radius: var(--rounded-md) 0 0 var(--rounded-md);
}
.header-sub-row .sub-item:last-child {
    border-radius: 0 var(--rounded-md) var(--rounded-md) 0;
}
.header-sub-row .sub-item:only-child { border-radius: var(--rounded-md); }

.sub-label {
    font-size: 11px;
    color: var(--color-body);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-weight: var(--font-weight-medium);
}
.sub-value {
    font-size: 15px;
    font-weight: var(--font-weight-semibold);
    color: var(--color-ink);
}
.header-grade { flex-shrink: 0; padding-top: 4px; }

.content-section {
    padding: 36px 0;
    border-bottom: 1px solid var(--color-hairline);
}
.content-section:last-child { border-bottom: none; }

.section-title {
    display: flex;
    align-items: center;
    gap: 10px;
    font-family: var(--font-display);
    font-size: 13px;
    font-weight: var(--font-weight-bold);
    color: var(--color-body);
    margin-bottom: 24px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
}
.section-title svg {
    color: var(--color-accent-mint);
    opacity: 0.8;
    flex-shrink: 0;
}

.kpi-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
}
@media (max-width: 760px) {
    .kpi-grid { grid-template-columns: 1fr; }
}

.kpi-card {
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-md);
    padding: 24px;
    position: relative;
    overflow: hidden;
    transition: border-color var(--dur-fast) var(--ease), transform var(--dur-fast) var(--ease);
}
.kpi-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.07), transparent);
}
.kpi-card:hover {
    border-color: var(--color-hover-border);
    transform: translateY(-1px);
}
.kpi-card-alert { border-color: rgba(239, 68, 68, 0.3); }
.kpi-card-alert::before {
    background: linear-gradient(90deg, transparent, rgba(239, 68, 68, 0.4), transparent);
}

.kpi-card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 20px;
}
.kpi-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--rounded-sm);
    flex-shrink: 0;
}
.turnover-icon { background: rgba(96, 165, 250, 0.12); color: #60a5fa; }
.dos-icon     { background: rgba(34, 197, 94, 0.12); color: #22c55e; }
.dead-icon    { background: rgba(239, 68, 68, 0.12); color: #ef4444; }
.accuracy-icon{ background: rgba(139, 92, 246, 0.12); color: #a78bfa; }

.kpi-card-title {
    flex: 1;
    font-size: 12px;
    font-weight: var(--font-weight-bold);
    color: var(--color-body);
    text-transform: uppercase;
    letter-spacing: 0.07em;
}
.score-badge {
    font-family: var(--font-mono);
    font-size: 13px;
    font-weight: var(--font-weight-bold);
    padding: 4px 12px;
    border-radius: 20px;
    border: 1px solid;
    line-height: 1;
}
.kpi-card-body {
    display: flex;
    flex-direction: column;
    gap: 12px;
}
.kpi-big-value {
    font-size: 36px;
    font-weight: var(--font-weight-semibold);
    color: var(--color-ink);
    line-height: 1;
    letter-spacing: -0.02em;
}
.kpi-unit {
    font-size: 13px;
    font-weight: var(--font-weight-normal);
    color: var(--color-body);
    margin-left: 4px;
    letter-spacing: 0;
}
.kpi-formula {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--color-secondary);
}
.formula-part {
    background: var(--color-hover-bg);
    padding: 3px 10px;
    border-radius: 5px;
    color: var(--color-ink);
    font-size: 13px;
}
.formula-op {
    color: var(--color-body);
    font-weight: var(--font-weight-semibold);
    font-size: 15px;
}
.kpi-formula-label {
    font-size: 11px;
    color: var(--color-body);
    letter-spacing: 0.03em;
}
.kpi-progress-wrap { margin-top: 2px; }
.kpi-progress-track {
    height: 5px;
    background: var(--color-overlay-bg);
    border-radius: 3px;
    overflow: hidden;
}
.kpi-progress-fill {
    height: 100%;
    border-radius: 3px;
    transition: width var(--dur-spring) var(--ease-spring);
}
.dos-bar-area { padding: 2px 0; }

.dead-status-row { margin-bottom: 4px; }
.dead-detected {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 16px;
    font-weight: var(--font-weight-bold);
    color: var(--status-danger);
}
.dead-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background: var(--status-danger);
    flex-shrink: 0;
}
.dead-dot.pulse {
    animation: dot-pulse 1.6s ease-in-out infinite;
}

.dead-normal {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 16px;
    font-weight: var(--font-weight-semibold);
    color: var(--status-ok);
}
.normal-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background: var(--status-ok);
    flex-shrink: 0;
}
.dead-value-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    background: var(--color-hover-bg);
    border-radius: var(--rounded-sm);
    border: 1px solid var(--color-hairline);
}
.dead-val-label { font-size: 13px; color: var(--color-body); }
.dead-val {
    font-size: 18px;
    font-weight: var(--font-weight-bold);
    color: var(--status-danger);
}
.dead-val-ok { font-size: 13px; color: var(--color-secondary); }

.accuracy-main { margin-bottom: 4px; }
.accuracy-discrepancy {
    font-size: 36px;
    font-weight: var(--font-weight-semibold);
    color: var(--status-ok);
    line-height: 1;
    letter-spacing: -0.02em;
}
.accuracy-discrepancy.has-diff { color: var(--status-warn); }

.accuracy-counts {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 16px;
    background: var(--color-hover-bg);
    border-radius: var(--rounded-sm);
    border: 1px solid var(--color-hairline);
}
.accuracy-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.acc-label { font-size: 12px; color: var(--color-body); }
.acc-value { font-size: 14px; color: var(--color-secondary); }
.diff-row {
    border-top: 1px solid var(--color-hairline);
    padding-top: 8px;
    margin-top: 2px;
}

.flow-diagram {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 28px 24px;
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-md);
    flex-wrap: wrap;
    margin-bottom: 16px;
}
.flow-node {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 18px 28px;
    background: var(--color-hover-bg);
    border-radius: var(--rounded-md);
    border: 1px solid var(--color-hairline);
    min-width: 110px;
    transition: border-color var(--dur-fast) var(--ease);
}
.flow-node:hover { border-color: var(--color-hover-border); }
.flow-node.flow-in {
    border-color: rgba(34, 197, 94, 0.25);
    background: rgba(34, 197, 94, 0.04);
}
.flow-node.flow-out {
    border-color: rgba(239, 68, 68, 0.25);
    background: rgba(239, 68, 68, 0.04);
}
.flow-node.flow-remain {
    border-color: rgba(0, 212, 184, 0.3);
    background: rgba(0, 212, 184, 0.05);
}

.flow-label {
    font-size: 12px;
    color: var(--color-body);
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    white-space: nowrap;
}
.flow-value {
    font-size: 22px;
    font-weight: var(--font-weight-bold);
    color: var(--color-ink);
    letter-spacing: -0.01em;
}
.flow-value.positive { color: var(--status-ok); }
.flow-value.negative { color: var(--status-danger); }
.flow-arrow {
    color: var(--color-body);
    opacity: 0.35;
    flex-shrink: 0;
}

.movement-extra {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 14px;
}
@media (max-width: 760px) {
    .movement-extra { grid-template-columns: 1fr 1fr; }
}

.movement-stat {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 16px 18px;
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-md);
}
.movement-stat-label {
    font-size: 11px;
    color: var(--color-body);
    text-transform: uppercase;
    letter-spacing: 0.07em;
    font-weight: var(--font-weight-semibold);
}
.movement-stat-value {
    font-size: 15px;
    font-weight: var(--font-weight-semibold);
    color: var(--color-secondary);
}
.movement-stat-sub {
    font-size: 12px;
    color: var(--color-body);
    font-weight: var(--font-weight-normal);
}

.score-table {
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-md);
    overflow: hidden;
}
.score-header-row {
    display: grid;
    grid-template-columns: 2fr 0.8fr 0.8fr 3fr 1fr;
    gap: 16px;
    padding: 14px 24px;
    border-bottom: 1px solid var(--color-hairline);
    font-size: 11px;
    font-weight: var(--font-weight-bold);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--color-body);
}
.score-row {
    display: grid;
    grid-template-columns: 2fr 0.8fr 0.8fr 3fr 1fr;
    gap: 16px;
    padding: 16px 24px;
    align-items: center;
    border-bottom: 1px solid var(--color-hairline);
    transition: background var(--dur-fast) var(--ease);
}
.score-row:hover { background: var(--color-hover-bg); }
.score-col-name {
    font-size: 14px;
    color: var(--color-ink);
    font-weight: var(--font-weight-medium);
}
.score-col-weight { font-size: 13px; color: var(--color-body); }
.score-col-raw { font-size: 15px; font-weight: var(--font-weight-bold); }
.score-col-bar { display: flex; align-items: center; }

.score-bar-track {
    width: 100%;
    height: 6px;
    background: var(--color-overlay-bg);
    border-radius: 3px;
    overflow: hidden;
}
.score-bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width var(--dur-spring) var(--ease-spring);
}
.score-col-weighted {
    font-size: 14px;
    color: var(--color-secondary);
    text-align: right;
}

.score-total-row {
    display: grid;
    grid-template-columns: 2fr 0.8fr 0.8fr 3fr 1fr;
    gap: 16px;
    padding: 18px 24px;
    align-items: center;
    background: rgba(255, 255, 255, 0.025);
    border-top: 1px solid var(--color-hairline);
}
.score-total-row .score-col-name {
    font-weight: var(--font-weight-bold);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-size: 13px;
}
.total-track { height: 9px; }
.total-fill { height: 100%; }
.total-score {
    font-size: 20px;
    font-weight: var(--font-weight-extrabold);
    text-align: right;
    letter-spacing: -0.02em;
}

.expiry-count {
    font-size: 13px;
    color: var(--color-body);
    font-weight: var(--font-weight-normal);
    letter-spacing: 0;
    text-transform: none;
}
.expiry-table {
    background: var(--color-elevated-bg);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-md);
    overflow: hidden;
}
.expiry-header {
    display: grid;
    grid-template-columns: 1.8fr 1fr 1fr 1fr 1.1fr 1fr;
    gap: 12px;
    padding: 14px 24px;
    border-bottom: 1px solid var(--color-hairline);
    font-size: 11px;
    font-weight: var(--font-weight-bold);
    text-transform: uppercase;
    letter-spacing: 0.09em;
    color: var(--color-body);
}
.expiry-row {
    display: grid;
    grid-template-columns: 1.8fr 1fr 1fr 1fr 1.1fr 1fr;
    gap: 12px;
    padding: 14px 24px;
    align-items: center;
    border-bottom: 1px solid var(--color-hairline);
    font-size: 13.5px;
    color: var(--color-secondary);
    transition: background var(--dur-fast) var(--ease);
}
.expiry-row:last-child { border-bottom: none; }
.expiry-row:hover { background: var(--color-hover-bg); }

.lot-no {
    color: var(--color-ink);
    font-weight: var(--font-weight-medium);
}
.num { text-align: right; }

.text-expired { color: var(--status-danger); font-weight: var(--font-weight-bold); }
.text-critical { color: #c2410c; font-weight: var(--font-weight-bold); }
.text-warning { color: var(--status-warn); font-weight: var(--font-weight-medium); }

.expiry-status-chip {
    font-size: 11px;
    font-weight: var(--font-weight-semibold);
    padding: 4px 10px;
    border-radius: 20px;
    white-space: nowrap;
    display: inline-block;
    letter-spacing: 0.02em;
}
.expiry-status-chip.expired {
    background: rgba(239, 68, 68, 0.12);
    color: var(--status-danger);
    border: 1px solid rgba(239, 68, 68, 0.3);
}
.expiry-status-chip.critical {
    background: rgba(249, 115, 22, 0.12);
    color: #c2410c;
    border: 1px solid rgba(249, 115, 22, 0.3);
}
.expiry-status-chip.warning {
    background: rgba(245, 158, 11, 0.12);
    color: var(--status-warn);
    border: 1px solid rgba(245, 158, 11, 0.3);
}
.expiry-status-chip.safe {
    background: rgba(34, 197, 94, 0.12);
    color: var(--status-ok);
    border: 1px solid rgba(34, 197, 94, 0.3);
}

@media (max-width: 900px) {
    .page-container { padding: 0 24px; }
}
@media (max-width: 640px) {
    .page-container { padding: 0 16px; }
    .drug-header { flex-direction: column; gap: 24px; }
    .header-grade { display: flex; justify-content: center; }
    .header-sub-row { flex-direction: column; align-items: stretch; }
    .sub-item {
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        border-radius: var(--rounded-sm) !important;
        border: 1px solid var(--color-hairline) !important;
    }
    .sub-divider { display: none; }
    .drug-name { font-size: 20px; }
    .kpi-big-value, .accuracy-discrepancy { font-size: 28px; }
    .flow-diagram { flex-direction: column; gap: 6px; padding: 20px 16px; }
    .flow-arrow { transform: rotate(90deg); }
    .flow-node {
        width: 100%;
        flex-direction: row;
        justify-content: space-between;
        padding: 14px 18px;
        min-width: unset;
    }
    .movement-extra { grid-template-columns: 1fr 1fr; }
    .score-header-row, .score-row, .score-total-row {
        grid-template-columns: 1.6fr 0.6fr 0.7fr 2fr 0.8fr;
        gap: 8px;
        padding: 12px 16px;
    }
    .expiry-header, .expiry-row {
        grid-template-columns: 1fr 0.9fr 0.9fr 0.8fr 0.9fr 0.9fr;
        gap: 6px;
        padding: 12px 14px;
        font-size: 12px;
    }
}
</style>

