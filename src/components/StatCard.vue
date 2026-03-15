<template>
    <div class="stat-card" :class="variant">
        <div class="stat-header">
            <div class="stat-icon" v-html="icon" />
            <span class="stat-label">{{ label }}</span>
        </div>
        <div class="stat-body">
            <span class="stat-value mono">{{ value }}</span>
            <span v-if="unit" class="stat-unit">{{ unit }}</span>
        </div>
        <div v-if="sub" class="stat-sub">{{ sub }}</div>
        <div class="stat-glow" />
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
.stat-card {
    background: var(--bg-elevated);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-lg);
    padding: 24px;
    position: relative;
    overflow: hidden;
    transition: border-color var(--dur-fast) var(--ease);
    cursor: default;
}

.stat-card:hover {
    border-color: var(--border-med);
}

.stat-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
}

.stat-card.danger {
    border-color: rgba(239, 68, 68, 0.2);
}

.stat-card.danger::before {
    background: linear-gradient(90deg, transparent, rgba(239, 68, 68, 0.4), transparent);
}

.stat-card.warn {
    border-color: rgba(245, 158, 11, 0.2);
}

.stat-card.warn::before {
    background: linear-gradient(90deg, transparent, rgba(245, 158, 11, 0.4), transparent);
}

.stat-card.ok {
    border-color: rgba(34, 197, 94, 0.2);
}

.stat-card.ok::before {
    background: linear-gradient(90deg, transparent, rgba(34, 197, 94, 0.4), transparent);
}

.stat-card.info {
    border-color: rgba(59, 130, 246, 0.2);
}

.stat-card.info::before {
    background: linear-gradient(90deg, transparent, rgba(59, 130, 246, 0.4), transparent);
}

.stat-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 16px;
}

.stat-icon {
    color: var(--text-muted);
    opacity: 0.7;
    display: flex;
}

.stat-card.danger .stat-icon {
    color: var(--status-danger);
    opacity: 1;
}

.stat-card.warn .stat-icon {
    color: var(--status-warn);
    opacity: 1;
}

.stat-card.ok .stat-icon {
    color: var(--status-ok);
    opacity: 1;
}

.stat-card.info .stat-icon {
    color: var(--status-info);
    opacity: 1;
}

.stat-label {
    font-size: 12px;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-weight: 600;
}

.stat-body {
    display: flex;
    align-items: baseline;
    gap: 6px;
}

.stat-value {
    font-size: 32px;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1;
}

.stat-card.danger .stat-value {
    color: var(--status-danger);
}

.stat-card.warn .stat-value {
    color: var(--status-warn);
}

.stat-card.ok .stat-value {
    color: var(--status-ok);
}

.stat-card.info .stat-value {
    color: var(--status-info);
}

.stat-unit {
    font-size: 13px;
    color: var(--text-secondary);
}

.stat-sub {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 8px;
    line-height: 1.4;
}

.stat-glow {
    position: absolute;
    bottom: -20px;
    right: -20px;
    width: 100px;
    height: 100px;
    border-radius: 50%;
    filter: blur(36px);
    opacity: 0.07;
}

.stat-card.danger .stat-glow {
    background: var(--status-danger);
}

.stat-card.warn .stat-glow {
    background: var(--status-warn);
}

.stat-card.ok .stat-glow {
    background: var(--status-ok);
}

.stat-card.info .stat-glow {
    background: var(--status-info);
}
</style>
