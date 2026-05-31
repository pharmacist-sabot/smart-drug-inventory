<template>
    <!-- toggle-pill-group — hairline rail with pill buttons -->
    <div class="toggle-group-rail">
        <button
            v-for="f in filters"
            :key="f.key"
            class="toggle-pill"
            :class="[f.cls, { active: modelValue === f.key }]"
            @click="$emit('update:modelValue', f.key)"
        >
            <span class="pill-dot" />
            <span class="pill-label">{{ f.label }}</span>
            <span class="pill-count">{{ f.count }}</span>
        </button>
    </div>
</template>

<script setup lang="ts">
export interface FilterOption {
  key: string;
  label: string;
  count: number;
  cls: string;
}

defineProps<{ modelValue: string; filters: FilterOption[] }>();
defineEmits<{ 'update:modelValue': [value: string] }>();
</script>

<style scoped>
.toggle-group-rail {
    display: flex;
    gap: var(--space-xs);
    flex-wrap: wrap;
    padding: var(--space-xs);
    background: var(--color-hairline);
    border-radius: var(--rounded-sm);
    width: fit-content;
}

.toggle-pill {
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
    padding: 6px var(--space-lg);
    background: var(--color-canvas);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-xs);
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: var(--font-weight-medium);
    letter-spacing: 0.08px;
    text-transform: uppercase;
    color: var(--color-body);
    cursor: pointer;
    transition: all var(--dur-fast) var(--ease);
    white-space: nowrap;
}

.toggle-pill:hover {
    color: var(--color-ink);
    border-color: var(--color-hover-border);
}

.toggle-pill.active {
    background: var(--color-primary);
    color: var(--color-canvas);
    border-color: var(--color-primary);
}

.pill-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
    opacity: 0.5;
    flex-shrink: 0;
}

.toggle-pill.active .pill-dot { opacity: 0.8; }

.toggle-pill.danger .pill-dot { background: var(--status-danger); opacity: 1; }
.toggle-pill.warn   .pill-dot { background: var(--status-warn);   opacity: 1; }
.toggle-pill.ok     .pill-dot { background: var(--status-ok);     opacity: 1; }
.toggle-pill.info   .pill-dot { background: var(--status-info);   opacity: 1; }
.toggle-pill.dead   .pill-dot { background: var(--violet-600);    opacity: 1; }

.toggle-pill.active .pill-dot { background: var(--color-canvas); }

.pill-label { flex: 1; }

.pill-count {
    font-family: var(--font-mono);
    font-size: 10px;
    background: rgba(0, 0, 0, 0.08);
    border-radius: var(--rounded-full);
    padding: 1px 6px;
    min-width: 20px;
    text-align: center;
}

.toggle-pill.active .pill-count {
    background: rgba(255, 255, 255, 0.2);
}
</style>
