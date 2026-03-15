<template>
    <div class="app-shell">
        <!-- Ambient background -->
        <div class="ambient" aria-hidden="true">
            <div class="ambient-orb orb-1" />
            <div class="ambient-orb orb-2" />
            <div class="grid-lines" />
        </div>

        <!-- Sidebar -->
        <aside class="sidebar">
            <div class="sidebar-logo">
                <div class="logo-icon">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none">
                        <path d="M12 2L2 7v10l10 5 10-5V7L12 2z" stroke="currentColor" stroke-width="1.5"
                            stroke-linejoin="round" />
                        <path d="M12 22V12M2 7l10 5 10-5" stroke="currentColor" stroke-width="1.5" />
                    </svg>
                </div>
                <div class="logo-text">
                    <span class="logo-main">PHARMA</span>
                    <span class="logo-sub">ANALYTICS</span>
                </div>
                <!-- spacer keeps logo row taller on HiDPI -->
            </div>

            <nav class="sidebar-nav">
                <router-link to="/" class="nav-item" active-class="active" exact-active-class="active">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="1.8">
                        <rect x="3" y="3" width="7" height="7" rx="1.5" />
                        <rect x="14" y="3" width="7" height="7" rx="1.5" />
                        <rect x="3" y="14" width="7" height="7" rx="1.5" />
                        <rect x="14" y="14" width="7" height="7" rx="1.5" />
                    </svg>
                    <span>Dashboard</span>
                </router-link>

                <router-link to="/settings" class="nav-item" active-class="active">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="1.8">
                        <circle cx="12" cy="12" r="3" />
                        <path
                            d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
                    </svg>
                    <span>ตั้งค่า</span>
                </router-link>
            </nav>

            <div class="sidebar-footer">
                <div class="version-badge">v1.0.0</div>
                <div class="sidebar-label">Smart Drug Inventory</div>
                <div class="db-status" :class="dbStatusClass">
                    <span class="dot" />
                    <span>{{ dbStatusText }}</span>
                </div>
            </div>
        </aside>

        <!-- Main content -->
        <main class="main-content">
            <router-view v-slot="{ Component }">
                <transition name="page" mode="out-in">
                    <component :is="Component" @settings-saved="onSettingsSaved" />
                </transition>
            </router-view>
        </main>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { healthCheck } from '@/api'

// null = checking, true = ok, false = offline
const dbOk = ref<boolean | null>(null)
const dbChecking = ref(false)

const dbStatusClass = computed(() => {
    if (dbOk.value === null) return 'checking'
    return dbOk.value ? 'ok' : 'err'
})

const dbStatusText = computed(() => {
    if (dbOk.value === null) return 'Checking...'
    if (dbOk.value) return 'Connected'
    return 'Offline'
})

async function checkDb() {
    if (dbChecking.value) return
    dbChecking.value = true
    // Reset to null (checking) while we re-probe
    dbOk.value = null
    try {
        const h = await healthCheck()
        dbOk.value = h?.database?.status === 'ok'
    } catch {
        dbOk.value = false
    } finally {
        dbChecking.value = false
    }
}

// Called by Settings page after a successful save so we re-check connectivity
// with the newly-saved credentials immediately.
async function onSettingsSaved() {
    // Small delay to allow the backend to pick up the new config
    await new Promise((r) => setTimeout(r, 400))
    await checkDb()
}

onMounted(() => {
    checkDb()
})
</script>

<style scoped>
.app-shell {
    display: flex;
    min-height: 100vh;
    position: relative;
}

/* ── Ambient background ── */
.ambient {
    position: fixed;
    inset: 0;
    pointer-events: none;
    z-index: 0;
    overflow: hidden;
}

.ambient-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(120px);
    opacity: 0.06;
}

.orb-1 {
    width: 600px;
    height: 600px;
    background: radial-gradient(circle, #00d4b8, transparent 70%);
    top: -200px;
    left: -100px;
}

.orb-2 {
    width: 500px;
    height: 500px;
    background: radial-gradient(circle, #3b82f6, transparent 70%);
    bottom: -100px;
    right: 10%;
}

.grid-lines {
    position: absolute;
    inset: 0;
    background-image: linear-gradient(rgba(255, 255, 255, 0.015) 1px, transparent 1px),
        linear-gradient(90deg, rgba(255, 255, 255, 0.015) 1px, transparent 1px);
    background-size: 40px 40px;
}

/* ── Sidebar ── */
.sidebar {
    width: 234px;
    min-height: 100vh;
    background: var(--bg-surface);
    border-right: 1px solid var(--border-soft);
    display: flex;
    flex-direction: column;
    position: sticky;
    top: 0;
    height: 100vh;
    z-index: 10;
    flex-shrink: 0;
}

.sidebar-logo {
    display: flex;
    align-items: center;
    gap: 13px;
    padding: 26px 22px 22px;
    border-bottom: 1px solid var(--border-dim);
}

.logo-icon {
    width: 40px;
    height: 40px;
    background: var(--accent-dim);
    border: 1px solid rgba(0, 212, 184, 0.2);
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    color: var(--accent);
    flex-shrink: 0;
}

.logo-text {
    display: flex;
    flex-direction: column;
    line-height: 1;
}

.logo-main {
    font-family: var(--font-display);
    font-size: 14px;
    font-weight: 700;
    letter-spacing: 0.15em;
    color: var(--text-primary);
}

.logo-sub {
    font-family: var(--font-mono);
    font-size: 10px;
    font-weight: 400;
    letter-spacing: 0.2em;
    color: var(--text-muted);
    margin-top: 3px;
}

.sidebar-nav {
    padding: 18px 14px;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 3px;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: 11px;
    padding: 11px 14px;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 14px;
    font-weight: 500;
    transition: all var(--dur-fast) var(--ease);
    border: 1px solid transparent;
}

.nav-item:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
    text-decoration: none;
}

.nav-item.active {
    color: var(--accent);
    background: var(--accent-glow);
    border: 1px solid rgba(0, 212, 184, 0.15);
}

.sidebar-footer {
    padding: 18px 22px;
    border-top: 1px solid var(--border-dim);
}

.sidebar-label {
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 8px;
    font-family: var(--font-mono);
}

.version-badge {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-muted);
    margin-bottom: 5px;
    opacity: 0.5;
}

.db-status {
    display: flex;
    align-items: center;
    gap: 7px;
    font-family: var(--font-mono);
    font-size: 12px;
    transition: color 0.3s ease;
}

.db-status.ok {
    color: var(--status-ok);
}

.db-status.err {
    color: var(--status-danger);
}

.db-status.checking {
    color: var(--text-muted);
}

.dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: currentColor;
    flex-shrink: 0;
    transition: background 0.3s ease;
}

.db-status.ok .dot {
    animation: pulse-ring 2s ease-in-out infinite;
}

/* ── Main content ── */
.main-content {
    flex: 1;
    min-width: 0;
    position: relative;
    z-index: 1;
}

/* ── Page transition ── */
.page-enter-active,
.page-leave-active {
    transition:
        opacity 200ms ease,
        transform 200ms ease;
}

.page-enter-from {
    opacity: 0;
    transform: translateY(8px);
}

.page-leave-to {
    opacity: 0;
    transform: translateY(-4px);
}

@keyframes pulse-ring {
    0% {
        transform: scale(0.8);
        box-shadow: 0 0 0 0 rgba(0, 212, 184, 0.7);
    }

    70% {
        transform: scale(1);
        box-shadow: 0 0 0 10px rgba(0, 212, 184, 0);
    }

    100% {
        transform: scale(0.8);
        box-shadow: 0 0 0 0 rgba(0, 212, 184, 0);
    }
}
</style>
