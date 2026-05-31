<template>
    <div class="settings-page">
        <header class="page-header">
            <h1 class="page-title">ตั้งค่าระบบ</h1>
            <p class="page-subtitle">กำหนดค่าการเชื่อมต่อฐานข้อมูลและค่าเริ่มต้นของแอปพลิเคชัน</p>
        </header>

        <!-- Status Messages -->
        <Transition name="msg">
            <div v-if="statusMessage" class="status-banner" :class="statusType">
                <span class="status-icon" v-html="statusIcon" />
                <span class="status-text">{{ statusMessage }}</span>
                <button class="status-dismiss" @click="clearStatus">×</button>
            </div>
        </Transition>

        <div class="settings-grid">
            <!-- Database Connection Section -->
            <section class="settings-card">
                <div class="card-header">
                    <div class="card-icon">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                            <ellipse cx="12" cy="5" rx="9" ry="3" />
                            <path d="M3 5v14c0 1.66 4.03 3 9 3s9-1.34 9-3V5" />
                            <path d="M3 12c0 1.66 4.03 3 9 3s9-1.34 9-3" />
                        </svg>
                    </div>
                    <div>
                        <h2 class="card-title">การเชื่อมต่อฐานข้อมูล</h2>
                        <p class="card-desc">ตั้งค่าการเชื่อมต่อ SQL Server</p>
                    </div>
                </div>

                <div class="form-body">
                    <div class="form-row two-col">
                        <div class="form-group">
                            <label class="form-label" for="db-server">Server</label>
                            <input id="db-server" v-model="form.db.server" type="text" class="form-input"
                                placeholder="localhost" spellcheck="false" />
                        </div>
                        <div class="form-group">
                            <label class="form-label" for="db-port">Port</label>
                            <input id="db-port" v-model.number="form.db.port" type="number" class="form-input mono"
                                placeholder="1433" min="1" max="65535" />
                        </div>
                    </div>

                    <div class="form-group">
                        <label class="form-label" for="db-name">Database Name</label>
                        <input id="db-name" v-model="form.db.database" type="text" class="form-input"
                            placeholder="DrugInventoryDB" spellcheck="false" />
                    </div>

                    <div class="form-row two-col">
                        <div class="form-group">
                            <label class="form-label" for="db-user">Username</label>
                            <input id="db-user" v-model="form.db.username" type="text" class="form-input"
                                placeholder="sa" spellcheck="false" :disabled="form.db.use_windows_auth" />
                        </div>
                        <div class="form-group">
                            <label class="form-label" for="db-pass">Password</label>
                            <input id="db-pass" v-model="form.db.password" type="password" class="form-input"
                                placeholder="••••••••" :disabled="form.db.use_windows_auth" />
                        </div>
                    </div>

                    <div class="form-row toggles">
                        <div class="toggle-group">
                            <label class="toggle-label" for="db-winauth">
                                <span class="toggle-switch">
                                    <input id="db-winauth" v-model="form.db.use_windows_auth" type="checkbox"
                                        class="toggle-input" />
                                    <span class="toggle-track">
                                        <span class="toggle-thumb" />
                                    </span>
                                </span>
                                <span class="toggle-text">Use Windows Auth</span>
                            </label>
                        </div>

                        <div class="toggle-group">
                            <label class="toggle-label" for="db-trustcert">
                                <span class="toggle-switch">
                                    <input id="db-trustcert" v-model="form.db.trust_cert" type="checkbox"
                                        class="toggle-input" />
                                    <span class="toggle-track">
                                        <span class="toggle-thumb" />
                                    </span>
                                </span>
                                <span class="toggle-text">Trust Server Certificate</span>
                            </label>
                        </div>
                    </div>

                    <div class="form-group">
                        <label class="form-label" for="db-timeout">Connection Timeout</label>
                        <div class="input-with-unit">
                            <input id="db-timeout" v-model.number="form.db.connect_timeout_secs" type="number"
                                class="form-input mono" placeholder="30" min="1" max="300" />
                            <span class="input-unit">วินาที (seconds)</span>
                        </div>
                    </div>

                    <div class="form-actions">
                        <button class="btn btn-secondary" :disabled="testingConnection" @click="handleTestConnection">
                            <svg v-if="!testingConnection" width="16" height="16" viewBox="0 0 24 24" fill="none"
                                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                                <polyline points="22 4 12 14.01 9 11.01" />
                            </svg>
                            <span v-if="testingConnection" class="spinner" />
                            <span>{{ testingConnection ? 'กำลังทดสอบ...' : 'ทดสอบการเชื่อมต่อ' }}</span>
                        </button>

                        <Transition name="result">
                            <span v-if="testResult" class="test-result" :class="testResultType">
                                {{ testResult }}
                            </span>
                        </Transition>
                    </div>
                </div>
            </section>

            <!-- Application Defaults Section -->
            <section class="settings-card">
                <div class="card-header">
                    <div class="card-icon">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="3" />
                            <path
                                d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
                        </svg>
                    </div>
                    <div>
                        <h2 class="card-title">ค่าเริ่มต้นของแอปพลิเคชัน</h2>
                        <p class="card-desc">กำหนดค่าพารามิเตอร์เริ่มต้นสำหรับการวิเคราะห์</p>
                    </div>
                </div>

                <div class="form-body">
                    <div class="form-group">
                        <label class="form-label" for="stock-id">รหัสคลังยาหลัก (Stock ID)</label>
                        <p class="form-hint">DEPT_ID ของคลังยาใหญ่ในระบบ เช่น STOCK1</p>
                        <input id="stock-id" v-model="form.default_stock_id" type="text" class="form-input mono"
                            placeholder="STOCK1" spellcheck="false" />
                    </div>

                    <div class="form-group">
                        <label class="form-label" for="rolling-months">Default Rolling Months</label>
                        <p class="form-hint">จำนวนเดือนย้อนหลังสำหรับคำนวณอัตราการใช้ยา (1–12)</p>
                        <div class="input-with-unit">
                            <input id="rolling-months" v-model.number="form.default_rolling_months" type="number"
                                class="form-input mono" min="1" max="12" placeholder="6" />
                            <span class="input-unit">เดือน (months)</span>
                        </div>
                    </div>

                    <div class="form-group">
                        <label class="form-label" for="expiry-days">Default Expiry Warning Days</label>
                        <p class="form-hint">จำนวนวันก่อนหมดอายุที่จะแสดงคำเตือน (1–365)</p>
                        <div class="input-with-unit">
                            <input id="expiry-days" v-model.number="form.default_expiry_days" type="number"
                                class="form-input mono" min="1" max="365" placeholder="180" />
                            <span class="input-unit">วัน (days)</span>
                        </div>
                    </div>
                </div>
            </section>
        </div>

        <!-- Save Button -->
        <div class="page-actions">
            <button class="btn btn-primary" :disabled="saving" @click="handleSave">
                <svg v-if="!saving" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
                    <polyline points="17 21 17 13 7 13 7 21" />
                    <polyline points="7 3 7 8 15 8" />
                </svg>
                <span v-if="saving" class="spinner" />
                <span>{{ saving ? 'กำลังบันทึก...' : 'บันทึกการตั้งค่า' }}</span>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { getSettings, saveSettings, testDbConnection } from '@/api'
import type { AppSettings, DbConfig } from '@/types'

const emit = defineEmits<{ 'settings-saved': [] }>()

const loading = ref(true)
const saving = ref(false)
const testingConnection = ref(false)

const statusMessage = ref('')
const statusType = ref<'success' | 'error' | 'info'>('info')
const testResult = ref('')
const testResultType = ref<'success' | 'error'>('success')

const form = reactive<AppSettings>({
    db: {
        server: '',
        port: 1433,
        database: '',
        username: '',
        password: '',
        use_windows_auth: false,
        trust_cert: false,
        connect_timeout_secs: 30,
    },
    default_rolling_months: 6,
    default_expiry_days: 180,
    default_stock_id: 'STOCK1',
})

const statusIcon = ref('')

function setStatus(message: string, type: 'success' | 'error' | 'info') {
    statusMessage.value = message
    statusType.value = type
    if (type === 'success') {
        statusIcon.value = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>'
    } else if (type === 'error') {
        statusIcon.value = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>'
    } else {
        statusIcon.value = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>'
    }
}

function clearStatus() {
    statusMessage.value = ''
}

function populateForm(settings: AppSettings) {
    form.db.server = settings.db.server
    form.db.port = settings.db.port
    form.db.database = settings.db.database
    form.db.username = settings.db.username
    form.db.password = settings.db.password
    form.db.use_windows_auth = settings.db.use_windows_auth
    form.db.trust_cert = settings.db.trust_cert
    form.db.connect_timeout_secs = settings.db.connect_timeout_secs
    form.default_rolling_months = settings.default_rolling_months
    form.default_expiry_days = settings.default_expiry_days
    form.default_stock_id = settings.default_stock_id || 'STOCK1'
}

onMounted(async () => {
    try {
        const settings = await getSettings()
        populateForm(settings)
    } catch (err: any) {
        setStatus(`ไม่สามารถโหลดการตั้งค่าได้: ${err?.message ?? err}`, 'error')
    } finally {
        loading.value = false
    }
})

async function handleTestConnection() {
    testResult.value = ''
    testingConnection.value = true

    const dbConfig: DbConfig = { ...form.db }

    try {
        const result = await testDbConnection(dbConfig)
        testResult.value = result || 'เชื่อมต่อสำเร็จ!'
        testResultType.value = 'success'
        // Notify App shell so the status dot updates immediately
        emit('settings-saved')
    } catch (err: any) {
        testResult.value = `การเชื่อมต่อล้มเหลว: ${err?.message ?? err}`
        testResultType.value = 'error'
    } finally {
        testingConnection.value = false
    }
}

async function handleSave() {
    saving.value = true
    clearStatus()

    const settingsToSave: AppSettings = {
        db: { ...form.db },
        default_rolling_months: form.default_rolling_months,
        default_expiry_days: form.default_expiry_days,
        default_stock_id: form.default_stock_id.trim() || 'STOCK1',
    }

    try {
        await saveSettings(settingsToSave)
        setStatus('บันทึกการตั้งค่าเรียบร้อยแล้ว', 'success')
        // Notify App shell to re-check DB connectivity with new credentials
        emit('settings-saved')
    } catch (err: any) {
        setStatus(`ไม่สามารถบันทึกการตั้งค่าได้: ${err?.message ?? err}`, 'error')
    } finally {
        saving.value = false
    }
}
</script>

<style scoped>
.settings-page {
    max-width: 860px;
    margin: 0 auto;
    padding: 40px 32px 80px;
    background: var(--color-canvas);
    min-height: 100vh;
}

/* ── Page Header ── */
.page-header { margin-bottom: var(--space-4xl); }

.page-title {
    font-family: var(--font-display);
    font-size: 26px;
    font-weight: 500;
    color: var(--color-ink);
    letter-spacing: -0.022em;
    margin: 0;
}

.page-subtitle {
    font-size: 14px;
    color: var(--color-body);
    margin: var(--space-sm) 0 0;
    line-height: 1.5;
}

/* ── Status Banner ── */
.status-banner {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-lg) var(--space-2xl);
    border-radius: var(--rounded-sm);
    margin-bottom: var(--space-3xl);
    font-size: 14px;
    border: 1px solid;
    line-height: 1.5;
}

.status-banner.success { background: var(--status-ok-bg);     border-color: rgba(22,163,74,.2);   color: var(--status-ok); }
.status-banner.error   { background: var(--status-danger-bg); border-color: rgba(220,38,38,.2);  color: var(--status-danger); }
.status-banner.info    { background: var(--status-info-bg);   border-color: rgba(37,99,235,.15); color: var(--status-info); }

.status-icon   { display: flex; align-items: center; flex-shrink: 0; }
.status-text   { flex: 1; line-height: 1.4; }

.status-dismiss {
    background: none;
    border: none;
    color: inherit;
    opacity: 0.6;
    cursor: pointer;
    font-size: 18px;
    line-height: 1;
    padding: 2px 6px;
    border-radius: var(--rounded-xs);
    transition: opacity var(--dur-fast) var(--ease);
}
.status-dismiss:hover { opacity: 1; }

/* ── Settings Grid ── */
.settings-grid {
    display: flex;
    flex-direction: column;
    gap: var(--space-3xl);
}

/* ── Card ── */
.settings-card {
    background: var(--color-canvas);
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-sm);
    overflow: hidden;
}

.card-header {
    display: flex;
    align-items: center;
    gap: var(--space-lg);
    padding: var(--space-2xl) var(--space-3xl) 0;
    border-bottom: 1px solid var(--color-hairline);
    padding-bottom: var(--space-2xl);
}

.card-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: var(--rounded-sm);
    background: #f3f4f6;
    color: var(--color-ink);
    flex-shrink: 0;
}

.card-title {
    font-size: 15px;
    font-weight: 500;
    color: var(--color-ink);
    margin: 0;
    letter-spacing: -0.01em;
}

.card-desc {
    font-size: 13px;
    color: var(--color-body);
    margin: 3px 0 0;
}

/* ── Form ── */
.form-body {
    padding: var(--space-2xl) var(--space-3xl) var(--space-3xl);
    display: flex;
    flex-direction: column;
    gap: var(--space-2xl);
}

.form-row { display: flex; gap: var(--space-xl); }
.form-row.two-col > .form-group { flex: 1; }
.form-row.toggles { flex-wrap: wrap; gap: var(--space-3xl); }

.form-group { display: flex; flex-direction: column; gap: var(--space-sm); }

.form-label {
    font-family: var(--font-mono);
    font-size: 9px;
    font-weight: 500;
    color: var(--color-body);
    text-transform: uppercase;
    letter-spacing: 0.55px;
}

.form-hint {
    font-size: 13px;
    color: var(--color-body);
    margin: -2px 0 2px;
    line-height: 1.5;
}

.form-input {
    width: 100%;
    padding: 10px var(--space-lg);
    background: #fff;
    border: 1px solid var(--color-hairline);
    border-radius: var(--rounded-sm);
    color: var(--color-ink);
    font-family: var(--font-display);
    font-size: 14px;
    outline: none;
    transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
    box-sizing: border-box;
}

.form-input::placeholder { color: #9ca3af; }
.form-input:hover:not(:disabled) { border-color: #9ca3af; }
.form-input:focus { border-color: var(--color-ink); box-shadow: 0 0 0 2px rgba(0,0,0,0.06); }
.form-input:disabled { opacity: 0.4; cursor: not-allowed; background: #f9fafb; }
.form-input.mono { font-family: var(--font-mono); font-size: 13px; }

.form-input[type='number'] { -moz-appearance: textfield; }
.form-input[type='number']::-webkit-inner-spin-button,
.form-input[type='number']::-webkit-outer-spin-button { opacity: 0; }
.form-input[type='number']:hover::-webkit-inner-spin-button,
.form-input[type='number']:hover::-webkit-outer-spin-button { opacity: 1; }

/* ── Input with unit ── */
.input-with-unit { display: flex; align-items: center; gap: var(--space-md); }
.input-with-unit .form-input { max-width: 180px; }
.input-unit { font-size: 13px; color: var(--color-body); white-space: nowrap; }

/* ── Toggle Switch ── */
.toggle-group { display: flex; align-items: center; }
.toggle-label { display: flex; align-items: center; gap: var(--space-lg); cursor: pointer; user-select: none; }
.toggle-switch { position: relative; display: inline-flex; align-items: center; flex-shrink: 0; }
.toggle-input { position: absolute; opacity: 0; width: 0; height: 0; }

.toggle-track {
    position: relative;
    width: 46px;
    height: 26px;
    background: #e5e7eb;
    border: 1px solid var(--color-hairline);
    border-radius: 13px;
    transition: background var(--dur-fast) var(--ease), border-color var(--dur-fast) var(--ease);
}

.toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: #9ca3af;
    border-radius: 50%;
    transition: transform var(--dur-fast) var(--ease), background var(--dur-fast) var(--ease);
}

.toggle-input:checked + .toggle-track { background: var(--color-ink); border-color: var(--color-ink); }
.toggle-input:checked + .toggle-track .toggle-thumb { transform: translateX(20px); background: #fff; }
.toggle-input:focus-visible + .toggle-track { box-shadow: 0 0 0 2px rgba(0,0,0,0.1); }

.toggle-text { font-size: 14px; color: var(--color-body); line-height: 1.4; }

/* ── Buttons ── */
.btn {
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
    padding: 10px var(--space-2xl);
    border-radius: var(--rounded-sm);
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 500;
    letter-spacing: 0.08px;
    text-transform: uppercase;
    border: 1px solid transparent;
    cursor: pointer;
    transition: opacity var(--dur-fast) var(--ease);
    white-space: nowrap;
}

.btn:disabled { opacity: 0.45; cursor: not-allowed; }

.btn-primary { background: var(--color-ink); color: #fff; border-color: var(--color-ink); }
.btn-primary:hover:not(:disabled) { opacity: 0.85; }

.btn-secondary {
    background: #fff;
    color: var(--color-ink);
    border-color: var(--color-hairline);
}
.btn-secondary:hover:not(:disabled) { border-color: #9ca3af; }

/* ── Form Actions ── */
.form-actions {
    display: flex;
    align-items: center;
    gap: var(--space-lg);
    flex-wrap: wrap;
    border-top: 1px solid var(--color-hairline);
    margin-top: var(--space-sm);
    padding-top: var(--space-2xl);
}

.test-result { font-size: 13px; font-weight: 500; line-height: 1.5; max-width: 440px; }
.test-result.success { color: var(--status-ok); }
.test-result.error   { color: var(--status-danger); }

/* ── Page Actions ── */
.page-actions { margin-top: var(--space-3xl); display: flex; justify-content: flex-end; }

/* ── Spinner ── */
.spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid transparent;
    border-top-color: currentColor;
    border-left-color: currentColor;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

/* ── Transitions ── */
.msg-enter-active, .msg-leave-active { transition: all var(--dur-fast) var(--ease); }
.msg-enter-from, .msg-leave-to { opacity: 0; transform: translateY(-8px); }
.result-enter-active, .result-leave-active { transition: opacity var(--dur-fast) var(--ease); }
.result-enter-from, .result-leave-to { opacity: 0; }

/* ── Responsive ── */
@media (max-width: 560px) {
    .settings-page { padding: 24px 16px 56px; }
    .form-row.two-col { flex-direction: column; gap: var(--space-xl); }
    .form-row.toggles { flex-direction: column; gap: var(--space-lg); }
    .card-header { padding: var(--space-xl) var(--space-xl) var(--space-xl); }
    .form-body { padding: var(--space-xl) var(--space-xl) var(--space-2xl); }
    .input-with-unit .form-input { max-width: 130px; }
    .form-actions { flex-direction: column; align-items: flex-start; }
}
</style>
