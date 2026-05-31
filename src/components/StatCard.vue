<template>
    <div class="stat-tile" :class="variant">
        <div class="tile-eyebrow">
            <div class="tile-icon" v-html="icon" />
            <span>{{ label }}</span>
        </div>
        <div class="tile-body">
            <span class="tile-value">{{ value }}</span>
            <span v-if="unit" class="tile-unit">{{ unit }}</span>
        </div>
        <div v-if="sub" class="tile-sub">{{ sub }}</div>
    </div>
</template>

<script setup lang="ts">
withDefaults(
    defineProps<{
        label: string
        value: string | number
        unit?: string
        sub?: string
        icon?: string
        variant?: string
    }>(),
    { variant: 'default' },
)
</script>

<style scoped>
/* stats-card-tinted — pastel tinted stat tile */
.stat-tile {
    background: var(--color-hairline);
    border: 1px solid transparent;
    border-radius: var(--rounded-sm);
    padding: var(--space-3xl);
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
}

.stat-tile.danger {
    background: var(--tile-danger-bg);
    border-color: rgba(220, 38, 38, 0.15);
}
.stat-tile.warn {
    background: var(--tile-warn-bg);
    border-color: rgba(217, 119, 6, 0.15);
}
.stat-tile.ok {
    background: var(--tile-ok-bg);
    border-color: rgba(16, 185, 129, 0.15);
}
.stat-tile.info {
    background: var(--tile-info-bg);
    border-color: rgba(37, 99, 235, 0.15);
}

/* eyebrow row */
.tile-eyebrow {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 500;
    letter-spacing: 0.55px;
    text-transform: uppercase;
    color: var(--color-body);
}

.tile-icon {
    display: flex;
    align-items: center;
    opacity: 0.7;
}

/* tinted icon tones */
.stat-tile.danger .tile-eyebrow { color: var(--status-danger); }
.stat-tile.warn   .tile-eyebrow { color: var(--status-warn); }
.stat-tile.ok     .tile-eyebrow { color: var(--status-ok); }
.stat-tile.info   .tile-eyebrow { color: var(--status-info); }

/* value row */
.tile-body {
    display: flex;
    align-items: baseline;
    gap: var(--space-xs);
}

.tile-value {
    font-family: var(--font-mono);
    font-size: 40px;
    font-weight: 500;
    line-height: 1;
    letter-spacing: -0.8px;
    color: var(--color-ink);
}

.tile-unit {
    font-family: var(--font-display);
    font-size: 14px;
    color: var(--color-body);
}

/* sub text */
.tile-sub {
    font-family: var(--font-display);
    font-size: 13px;
    color: var(--color-body);
    line-height: 1.4;
}
</style>
