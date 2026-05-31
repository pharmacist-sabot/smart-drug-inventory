<template>
    <div class="app-shell">
        <!-- Sidebar (canvas-dark band) -->
        <aside class="sidebar">
            <!-- Logo -->
            <div class="sidebar-logo">
                <div class="logo-mark">
                    <!-- Pharmacy pill logo from public/logo.svg -->
                    <svg width="36" height="36" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <defs>
                            <radialGradient id="sb_bg" cx="50%" cy="50%" r="50%" fx="30%" fy="30%">
                                <stop offset="0%" stop-color="#ffffff"/>
                                <stop offset="100%" stop-color="#cbd5e1"/>
                            </radialGradient>
                            <linearGradient id="sb_pill_g" x1="9" y1="7" x2="15" y2="7" gradientUnits="userSpaceOnUse">
                                <stop offset="0%" stop-color="#10b981"/>
                                <stop offset="50%" stop-color="#34d399"/>
                                <stop offset="100%" stop-color="#059669"/>
                            </linearGradient>
                            <linearGradient id="sb_pill_y" x1="9" y1="12" x2="15" y2="12" gradientUnits="userSpaceOnUse">
                                <stop offset="0%" stop-color="#f59e0b"/>
                                <stop offset="50%" stop-color="#fbbf24"/>
                                <stop offset="100%" stop-color="#d97706"/>
                            </linearGradient>
                        </defs>
                        <circle cx="12" cy="12" r="11.5" fill="url(#sb_bg)" />
                        <circle cx="12" cy="12" r="11.5" stroke="#94a3b8" stroke-width="0.2" opacity="0.5"/>
                        <g opacity="0.9">
                            <path d="M12 2C6.48 2 2 6.48 2 12" stroke="#ef4444" stroke-width="2.8" stroke-linecap="round"/>
                            <path d="M2 12C2 17.52 6.48 22 12 22" stroke="#f59e0b" stroke-width="2.8" stroke-linecap="round"/>
                            <path d="M12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2" stroke="#10b981" stroke-width="3.8" stroke-linecap="round"/>
                        </g>
                        <rect x="9" y="7" width="6" height="10" rx="3" fill="url(#sb_pill_g)" />
                        <path d="M9 12H15V14C15 15.6569 13.6569 17 12 17C10.3431 17 9 15.6569 9 14V12Z" fill="url(#sb_pill_y)" />
                        <rect x="10" y="8" width="1.2" height="3" rx="0.6" fill="white" opacity="0.4" />
                    </svg>
                </div>
                <div class="logo-text">
                    <span class="logo-name">ONE PHARM</span>
                    <span class="logo-sub">SMART INVENTORY</span>
                </div>
            </div>

            <!-- Brand gradient divider -->
            <div class="gradient-rule" aria-hidden="true" />

            <!-- Navigation -->
            <nav class="sidebar-nav">
                <span class="nav-section-label">NAVIGATION</span>

                <router-link to="/" class="nav-item" active-class="active" exact-active-class="active">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="3" y="3" width="7" height="7" rx="1.5"/>
                        <rect x="14" y="3" width="7" height="7" rx="1.5"/>
                        <rect x="3" y="14" width="7" height="7" rx="1.5"/>
                        <rect x="14" y="14" width="7" height="7" rx="1.5"/>
                    </svg>
                    <span>DASHBOARD</span>
                </router-link>

                <router-link to="/settings" class="nav-item" active-class="active">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="3"/>
                        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
                    </svg>
                    <span>SETTINGS</span>
                </router-link>
            </nav>

            <!-- Sidebar footer -->
            <div class="sidebar-footer">
                <div class="db-status" :class="dbStatusClass">
                    <span class="db-dot" />
                    <span class="db-label">{{ dbStatusText }}</span>
                </div>
                <div class="version-label">v1.0.0</div>
            </div>
        </aside>

        <!-- Main content (canvas — light surface) -->
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

const dbOk = ref<boolean | null>(null)
const dbChecking = ref(false)

const dbStatusClass = computed(() => {
    if (dbOk.value === null) return 'checking'
    return dbOk.value ? 'ok' : 'err'
})

const dbStatusText = computed(() => {
    if (dbOk.value === null) return 'CHECKING...'
    if (dbOk.value) return 'CONNECTED'
    return 'OFFLINE'
})

async function checkDb() {
    if (dbChecking.value) return
    dbChecking.value = true
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

async function onSettingsSaved() {
    await new Promise((r) => setTimeout(r, 400))
    await checkDb()
}

onMounted(() => { checkDb() })
</script>

<style scoped>
.app-shell {
    display: flex;
    min-height: 100vh;
    background: var(--color-canvas);
}

.sidebar {
    width: var(--sidebar-width);
    min-height: 100vh;
    background: var(--color-canvas-dark);
    border-right: 1px solid var(--color-surface-dark-soft);
    display: flex;
    flex-direction: column;
    position: sticky;
    top: 0;
    height: 100vh;
    z-index: var(--z-sidebar);
    flex-shrink: 0;
}

.sidebar-logo {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 24px 20px 20px;
}
.logo-mark {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
}
.logo-text {
    display: flex;
    flex-direction: column;
    line-height: 1;
}
.logo-name {
    font-family: var(--font-mono);
    font-size: 13px;
    font-weight: var(--font-weight-medium);
    letter-spacing: 0.15em;
    color: var(--color-on-dark);
}
.logo-sub {
    font-family: var(--font-mono);
    font-size: 9px;
    font-weight: var(--font-weight-normal);
    letter-spacing: 0.12em;
    color: var(--color-on-dark-muted);
    margin-top: 4px;
}

.gradient-rule {
    height: 2px;
    background: linear-gradient(90deg,
        var(--color-accent-orange),
        var(--color-accent-magenta),
        var(--color-accent-periwinkle)
    );
    flex-shrink: 0;
}

.sidebar-nav {
    padding: 20px 12px;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
}
.nav-section-label {
    font-family: var(--font-mono);
    font-size: 9px;
    font-weight: var(--font-weight-medium);
    letter-spacing: 0.55px;
    text-transform: uppercase;
    color: var(--color-on-dark-muted);
    padding: 0 10px;
    margin-bottom: 8px;
    display: block;
}
.nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: var(--rounded-sm);
    color: var(--color-on-dark-muted);
    text-decoration: none;
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: var(--font-weight-medium);
    letter-spacing: 0.08em;
    text-transform: uppercase;
    transition: color var(--dur-fast) var(--ease), background var(--dur-fast) var(--ease);
    border: 1px solid transparent;
}
.nav-item:hover {
    color: var(--color-on-dark);
    background: rgba(255, 255, 255, 0.06);
    text-decoration: none;
}
.nav-item.active {
    color: var(--color-on-dark);
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.12);
}

.sidebar-footer {
    padding: 16px 20px;
    border-top: 1px solid var(--color-surface-dark-soft);
    display: flex;
    flex-direction: column;
    gap: 6px;
}
.db-status {
    display: flex;
    align-items: center;
    gap: 7px;
    font-family: var(--font-mono);
    font-size: 10px;
    font-weight: var(--font-weight-medium);
    letter-spacing: 0.08em;
}
.db-status.ok    { color: var(--green-400); }
.db-status.err   { color: var(--red-400); }
.db-status.checking { color: var(--color-on-dark-muted); }

.db-dot {
    width: var(--db-dot-size);
    height: var(--db-dot-size);
    border-radius: 50%;
    background: currentColor;
    flex-shrink: 0;
}
.db-status.ok .db-dot {
    animation: pulse-dot 2s ease-in-out infinite;
}
.db-label { text-transform: uppercase; }

.version-label {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--color-on-dark-muted);
    opacity: 0.4;
}

.main-content {
    flex: 1;
    min-width: 0;
    background: var(--color-canvas);
    overflow-y: auto;
}

.page-enter-active,
.page-leave-active {
    transition: opacity 0.18s var(--ease), transform 0.18s var(--ease);
}
.page-enter-from { opacity: 0; transform: translateY(6px); }
.page-leave-to   { opacity: 0; transform: translateY(-4px); }
</style>
