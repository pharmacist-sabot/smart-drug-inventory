<template>
    <div class="grade-ring" :style="{ width: size + 'px', height: size + 'px' }">
        <svg :width="size" :height="size" :viewBox="`0 0 ${size} ${size}`" style="overflow: visible">
            <circle :cx="size / 2" :cy="size / 2" :r="r" fill="none" stroke="rgba(255,255,255,0.06)"
                :stroke-width="strokeW" />
            <circle :cx="size / 2" :cy="size / 2" :r="r" fill="none" :stroke="gradeColor" :stroke-width="strokeW"
                stroke-linecap="round" :stroke-dasharray="circumference" :stroke-dashoffset="offset"
                :transform="`rotate(-90 ${size / 2} ${size / 2})`"
                :style="{ transition: 'stroke-dashoffset 1s cubic-bezier(0.16,1,0.3,1)', filter: `drop-shadow(0 0 5px ${gradeColor}80)` }" />
        </svg>
        <div class="ring-inner">
            <span class="ring-score mono">{{ displayScore }}</span>
            <span class="ring-grade" :style="{ color: gradeColor }">{{ grade }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, watch } from 'vue'
import type { Grade } from '@/types'

const props = withDefaults(
    defineProps<{ score: number; grade: Grade; size?: number; strokeW?: number }>(),
    { size: 100, strokeW: 6 },
)

const GRADE_COLORS: Record<Grade, string> = { A: '#22c55e', B: '#84cc16', C: '#f59e0b', D: '#ef4444' }
const gradeColor = computed(() => GRADE_COLORS[props.grade] ?? '#ef4444')

const r = computed(() => props.size / 2 - props.strokeW / 2 - 2)
const circumference = computed(() => 2 * Math.PI * r.value)

const animScore = ref(0)
const offset = computed(() => circumference.value * (1 - animScore.value / 100))
const displayScore = computed(() => Math.round(animScore.value))

onMounted(() => {
    setTimeout(() => { animScore.value = props.score }, 120)
})
watch(() => props.score, (v) => { animScore.value = v })
</script>

<style scoped>
.grade-ring {
    position: relative;
    flex-shrink: 0;
}

.ring-inner {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    pointer-events: none;
}

.ring-score {
    font-size: 26px;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1;
    letter-spacing: -0.02em;
}

.ring-grade {
    font-family: var(--font-display);
    font-size: 20px;
    font-weight: 700;
    line-height: 1;
    margin-top: 4px;
    letter-spacing: 0.02em;
}
</style>
