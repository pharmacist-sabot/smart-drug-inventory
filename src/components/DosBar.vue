<template>
    <div class="dos-bar-wrap">
        <div class="dos-track">
            <div class="zone-marker" style="left:11.7%"><span>7</span></div>
            <div class="zone-marker" style="left:25%"><span>15</span></div>
            <div class="zone-marker" style="left:100%"><span>60</span></div>
            <div class="dos-fill" :style="{ width: fillPct + '%', background: color }" />
        </div>
        <div class="dos-labels">
            <span class="dos-status-label" :style="{ color }">{{ label }}</span>
            <span class="dos-value mono">{{ dosDisplay }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { DosStatus } from '@/types'

const props = withDefaults(
    defineProps<{ dos: number | null; status: DosStatus }>(),
    { dos: null, status: 'normal' },
)

const MAX_VIS = 120

const fillPct = computed(() => {
    if (props.dos === null) return 0
    return Math.min((props.dos / MAX_VIS) * 100, 100)
})

const colorMap: Record<DosStatus, string> = {
    stockout_risk: '#ef4444', low_stock: '#f59e0b', normal: '#22c55e', overstock: '#3b82f6',
}
const color = computed(() => colorMap[props.status] || '#8b9ab0')

const labelMap: Record<DosStatus, string> = {
    stockout_risk: 'เสี่ยง Stockout', low_stock: 'สต็อกน้อย', normal: 'ปกติ', overstock: 'เกินกำหนด',
}
const label = computed(() => labelMap[props.status] || '-')

const dosDisplay = computed(() =>
    props.dos === null ? 'ไม่มีการจ่าย' : `${props.dos} วัน`,
)
</script>

<style scoped>
.dos-bar-wrap {
    width: 100%;
}

.dos-track {
    position: relative;
    height: 5px;
    background: var(--bg-overlay);
    border-radius: 3px;
    overflow: visible;
    margin-bottom: 6px;
}

.dos-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.8s cubic-bezier(0.16, 1, 0.3, 1);
    opacity: 0.85;
}

.zone-marker {
    position: absolute;
    top: -2px;
    width: 1px;
    height: 9px;
    background: rgba(255, 255, 255, 0.15);
}

.zone-marker span {
    position: absolute;
    top: 11px;
    left: 50%;
    transform: translateX(-50%);
    font-family: var(--font-mono);
    font-size: 9px;
    color: var(--text-muted);
    white-space: nowrap;
}

.dos-labels {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 14px;
}

.dos-status-label {
    font-size: 11px;
    font-weight: 500;
}

.dos-value {
    font-size: 11px;
    color: var(--text-secondary);
}
</style>
