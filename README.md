# Smart Drug Inventory — Desktop Application

ระบบประเมินประสิทธิภาพคลังยา (Drug Inventory Analytics) แบบ Desktop Application  
สร้างด้วย **Tauri (Rust)** + **Vue 3 (TypeScript)** เชื่อมต่อกับ SQL Server (INVS) แบบ read-only

## Tech Stack

| Layer    | Technology                                         |
| -------- | -------------------------------------------------- |
| Backend  | **Rust** via Tauri 2 — IPC commands                |
| Frontend | **Vue 3** (Composition API, `<script setup>`)      |
| Language | **TypeScript** (strict mode)                       |
| State    | **Pinia** — reactive store                         |
| Router   | **Vue Router 5** — SPA routing                     |
| Database | **SQL Server** via `tiberius` (TDS protocol)       |
| Pool     | **bb8** + **bb8-tiberius** — async connection pool |
| Build    | **Vite 7** (frontend) + **Cargo** (backend)        |
| Font     | Prompt + Sarabun + DM Mono (Google Fonts)          |

## โครงสร้างโปรเจกต์

```
smart-drug-inventory/
├── index.html                 ← HTML entry point
├── package.json               ← Node dependencies
├── vite.config.ts             ← Vite build config
├── tsconfig.json              ← TypeScript config
├── src/
│   ├── main.ts                ← Vue app bootstrap
│   ├── App.vue                ← Shell + sidebar navigation
│   ├── types/index.ts         ← Shared TypeScript interfaces
│   ├── api/index.ts           ← Tauri invoke wrappers
│   ├── assets/main.css        ← Design system (CSS variables)
│   ├── router/index.ts        ← Vue Router routes
│   ├── stores/inventory.ts    ← Pinia store
│   ├── components/
│   │   ├── GradeRing.vue      ← Animated circular score
│   │   ├── StatCard.vue       ← KPI card with variants
│   │   ├── DosBar.vue         ← DOS visualization bar
│   │   ├── DrugTable.vue      ← Sortable/filterable table
│   │   └── FilterBar.vue      ← Filter chips
│   └── views/
│       ├── Dashboard.vue      ← หน้าหลัก
│       ├── DrugDetail.vue     ← รายละเอียดรายยา
│       └── Settings.vue       ← ตั้งค่าการเชื่อมต่อ DB
└── src-tauri/
    ├── Cargo.toml             ← Rust dependencies
    ├── tauri.conf.json        ← Tauri app config
    ├── build.rs               ← Tauri build script
    └── src/
        ├── main.rs            ← Binary entry point
        ├── lib.rs             ← Tauri app setup + command registration
        ├── commands.rs        ← Tauri IPC commands (frontend ↔ backend)
        ├── database.rs        ← SQL Server connection pool (tiberius + bb8)
        ├── queries.rs         ← SQL queries for INVS schema
        ├── kpi.rs             ← KPI calculation logic (23 unit tests)
        └── settings.rs        ← Persistent app settings (JSON file)
```

## การติดตั้ง

### Prerequisites

1. **Rust** (1.77.2+) — [rustup.rs](https://rustup.rs)
2. **Node.js** (20.19+ or 22.12+) — [nodejs.org](https://nodejs.org)
3. **Tauri CLI** — `npm install -g @tauri-apps/cli`
4. **SQL Server** — ฐานข้อมูล INVS ที่เข้าถึงได้ผ่าน TCP/IP (port 1433)

> **หมายเหตุ:** ไม่ต้องติดตั้ง ODBC Driver — ระบบใช้ **tiberius** (TDS protocol) เชื่อมต่อ SQL Server โดยตรง

### ติดตั้ง dependencies

```bash
# ติดตั้ง Node dependencies
npm install

# ตรวจสอบว่า Rust compile ได้
cd src-tauri && cargo check && cd ..
```

### รัน Development

```bash
npx tauri dev
```

จะเปิดหน้าต่างแอป Desktop พร้อม hot-reload สำหรับทั้ง frontend (Vite) และ backend (Cargo)

### Build Production

```bash
npx tauri build
```

ไฟล์ installer จะอยู่ใน `src-tauri/target/release/bundle/`

## การตั้งค่า

### ผ่าน UI (แนะนำ)

1. เปิดแอป
2. คลิก **"ตั้งค่า"** ที่ sidebar ซ้าย
3. กรอกข้อมูล SQL Server:
   - **Server** — ชื่อ/IP ของ SQL Server
   - **Port** — พอร์ต (default: 1433)
   - **Database** — ชื่อฐานข้อมูล (default: INVS)
   - **Username / Password** — SQL Server authentication
   - **Trust Certificate** — เปิดสำหรับ self-signed cert
4. กด **"ทดสอบการเชื่อมต่อ"** เพื่อตรวจสอบ
5. กด **"บันทึก"** เพื่อบันทึกการตั้งค่า

### ไฟล์ Settings

การตั้งค่าจะถูกบันทึกเป็น JSON ที่:

| OS      | Path                                                     |
| ------- | -------------------------------------------------------- |
| macOS   | `~/Library/Application Support/com.smartdrug.inventory/` |
| Windows | `%APPDATA%\smartdrug\inventory\data\`                    |
| Linux   | `~/.local/share/inventory/`                              |

## หน้าจอ

### Dashboard (`/`)

- เลือกคลัง / ปี พ.ศ. / เดือน แล้วกด "โหลดข้อมูล"
- แสดง Overall Score พร้อม Grade A–D
- DOS Distribution bar (Stockout / Low / Normal / Overstock)
- Top issues 3 ประเภท (Stockout, Overstock, Dead Stock)
- ตารางยาครบทุกรายการ พร้อม filter, search, sort, pagination

### Drug Detail (`/drug/:code`)

- KPI breakdown ทั้ง 5 ตัว พร้อมคะแนนแยก
- Movement flow: ยกมา → รับ → จ่าย → คงเหลือ
- Score breakdown พร้อม weight
- Expiry lots (ถ้ามี)

### Settings (`/settings`)

- ตั้งค่าการเชื่อมต่อ SQL Server
- ทดสอบการเชื่อมต่อ
- ค่า default สำหรับ Rolling Months และ Expiry Warning Days

## KPI Thresholds

| KPI        | เงื่อนไข                   | สถานะ            |
| ---------- | -------------------------- | ---------------- |
| DOS        | < 7 วัน                    | 🔴 Stockout Risk |
| DOS        | 7–14 วัน                   | 🟡 Low Stock     |
| DOS        | 15–60 วัน                  | 🟢 Normal        |
| DOS        | > 60 วัน                   | 🔵 Overstock     |
| Dead Stock | RM > 0, ไม่มีจ่าย ≥ 60 วัน | ☠️ Dead Stock    |
| Expiry     | ≤ 0 วัน                    | 🔴 Expired       |
| Expiry     | 1–30 วัน                   | 🟠 Critical      |
| Expiry     | 31–90 วัน                  | 🟡 Warning       |

### KPI Weights

| KPI            | Weight |
| -------------- | ------ |
| Turnover Rate  | 30%    |
| Days of Supply | 25%    |
| Dead Stock     | 20%    |
| Expiry Risk    | 15%    |
| Stock Accuracy | 10%    |

### Grade

| Grade | Score Range |
| ----- | ----------- |
| A     | ≥ 80        |
| B     | 60–79       |
| C     | 40–59       |
| D     | < 40        |

## Rolling Window DOS

ระบบใช้ Rolling Window สำหรับคำนวณ Days of Supply:

```
daily_usage = ROLLING_DIS_QTY / rolling_days
DOS         = RM_QTY / daily_usage
```

- **Display period** — ช่วงเวลาที่ผู้ใช้เลือก (ใช้สำหรับ Turnover, แสดงผลการเคลื่อนไหว)
- **Rolling window** — ย้อนหลัง N เดือน (default: 3) สำหรับคำนวณ DOS และ Dead Stock

ตัวอย่าง: ผู้ใช้เลือก มี.ค. 2568, rolling = 3 เดือน

- Display = มี.ค. 2568 (31 วัน)
- Rolling = ม.ค.–มี.ค. 2568 (90 วัน)
- DOS คำนวณจากอัตราจ่ายเฉลี่ย 90 วัน → ไม่ผันผวนตามเดือนที่ยังไม่สิ้นสุด

## Development

### Rust — ตรวจสอบ

```bash
cd src-tauri

# Format
cargo fmt

# Lint (pedantic)
cargo clippy -- -W clippy::all

# Unit tests (23 tests)
cargo test

# Check compilation
cargo check
```

### TypeScript — ตรวจสอบ

```bash
# Type check
npx vue-tsc --noEmit

# Build (includes type check)
npm run build
```

### Tauri Commands (IPC)

| Command               | คำอธิบาย                          |
| --------------------- | --------------------------------- |
| `health_check`        | ตรวจสอบ connection                |
| `get_warehouses`      | รายการคลังยาทั้งหมด               |
| `get_kpi_summary`     | KPI สรุประดับคลัง                 |
| `get_drug_kpi_list`   | รายการยาพร้อม KPI (รองรับ filter) |
| `get_drug_kpi_detail` | KPI รายละเอียดยา 1 รายการ         |
| `get_settings`        | ดึงการตั้งค่าปัจจุบัน             |
| `save_settings`       | บันทึกการตั้งค่าใหม่ + reconnect  |
| `test_db_connection`  | ทดสอบการเชื่อมต่อ DB              |

## ความปลอดภัย

- ระบบเป็น **read-only** — ไม่มีการเขียนข้อมูลลง INVS
- Password เก็บใน settings file บนเครื่อง local เท่านั้น
- ไม่มี network exposure — เป็น desktop app ที่เชื่อมต่อ DB โดยตรง
- ควรสร้าง SQL user แยกต่างหากสำหรับระบบนี้ ไม่ใช้ `sa`

## Migration Notes (จาก Web App)

| เดิม (Web App)         | ใหม่ (Desktop App)                      |
| ---------------------- | --------------------------------------- |
| Python FastAPI backend | Rust (Tauri) backend                    |
| pyodbc + ODBC Driver   | tiberius (TDS protocol, no ODBC needed) |
| REST API (HTTP)        | Tauri IPC (invoke)                      |
| Axios HTTP client      | `@tauri-apps/api/core` invoke           |
| Vue 3 + JavaScript     | Vue 3 + TypeScript (strict)             |
| `.env` configuration   | Settings UI + JSON file                 |
| Browser-based          | Native desktop window                   |
| CORS middleware        | ไม่จำเป็น (same process)                |
| Uvicorn server         | ไม่จำเป็น (embedded in app)             |
