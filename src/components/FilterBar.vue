<template>
    <div class="filter-bar">
        <button v-for="f in filters" :key="f.key" class="filter-btn" :class="[f.cls, { active: modelValue === f.key }]"
            @click="$emit('update:modelValue', f.key)">
            <span class="filter-dot" />
            <span>{{ f.label }}</span>
            <span class="filter-count mono">{{ f.count }}</span>
        </button>
    </div>
</template>

<script setup lang="ts">
export interface FilterOption {
    key: string
    label: string
    count: number
    cls: string
}

defineProps<{ modelValue: string; filters: FilterOption[] }>()
defineEmits<{ 'update:modelValue': [value: string] }>()
</script>

<style scoped>
.filter-bar {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
}

.filter-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-soft);
    border-radius: 20px;
    font-family: var(--font-body);
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--dur-fast) var(--ease);
    white-space: nowrap;
}

.filter-btn:hover {
    color: var(--text-primary);
    border-color: var(--border-med);
}

.filter-btn.active {
    background: var(--bg-overlay);
    color: var(--text-primary);
}

.filter-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--text-muted);
    flex-shrink: 0;
}

.filter-btn.all .filter-dot {
    background: var(--text-secondary);
}

.filter-btn.danger .filter-dot {
    background: var(--status-danger);
}

.filter-btn.warn .filter-dot {
    background: var(--status-warn);
}

.filter-btn.ok .filter-dot {
    background: var(--status-ok);
}

.filter-btn.info .filter-dot {
    background: var(--status-info);
}

.filter-btn.dead .filter-dot {
    background: #a855f7;
}

.filter-btn.danger.active {
    border-color: rgba(239, 68, 68, 0.4);
    background: rgba(239, 68, 68, 0.08);
    color: var(--status-danger);
}

.filter-btn.warn.active {
    border-color: rgba(245, 158, 11, 0.4);
    background: rgba(245, 158, 11, 0.08);
    color: var(--status-warn);
}

.filter-btn.ok.active {
    border-color: rgba(34, 197, 94, 0.4);
    background: rgba(34, 197, 94, 0.08);
    color: var(--status-ok);
}

.filter-btn.info.active {
    border-color: rgba(59, 130, 246, 0.4);
    background: rgba(59, 130, 246, 0.08);
    color: var(--status-info);
}

.filter-btn.dead.active {
    border-color: rgba(168, 85, 247, 0.4);
    background: rgba(168, 85, 247, 0.08);
    color: #a855f7;
}

.filter-count {
    background: rgba(255, 255, 255, 0.06);
    border-radius: 10px;
    padding: 2px 8px;
    font-size: 11px;
    font-family: var(--font-mono);
    min-width: 24px;
    text-align: center;
}
</style>
