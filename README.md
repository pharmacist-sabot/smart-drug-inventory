# OnePharm

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/suradet-ps/one-pharm)
[![Rust](https://img.shields.io/badge/Rust-1.85-orange.svg?logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2-purple.svg?logo=tauri)](https://v2.tauri.app)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

> **Drug Inventory Analytics** — A cross-platform desktop application for evaluating pharmacy stock efficiency, built with Tauri 2 (Rust) + Vue 3 (TypeScript), connecting to SQL Server (INVS) in read-only mode.

## Tech Stack

| Layer    | Technology                                         |
| -------- | -------------------------------------------------- |
| Backend  | Rust via Tauri 2 — IPC commands                    |
| Frontend | Vue 3 (Composition API, `<script setup>`)          |
| Language | TypeScript (strict)                                 |
| State    | Pinia                                               |
| Router   | Vue Router 5                                        |
| Database | SQL Server via `tiberius` (TDS protocol, no ODBC)  |
| Pool     | bb8 + bb8-tiberius                                  |
| Build    | Vite 8 (frontend) + Cargo (backend)                |

## Quick Start

### Prerequisites

- **Rust** (1.85+) — [rustup.rs](https://rustup.rs)
- **Node.js** (20.19+ or 22.12+) — [nodejs.org](https://nodejs.org)
- **SQL Server** — INVS database accessible via TCP/IP (port 1433)

### Setup & Run

```bash
npm install                    # Install Node dependencies
cd src-tauri && cargo check    # Verify Rust compiles
cd .. && npx tauri dev         # Launch with hot-reload
```

### Build

```bash
npx tauri build
```

Installer output: `src-tauri/target/release/bundle/`

## Project Structure

```
one-pharm/
├── index.html
├── package.json
├── vite.config.ts
├── tsconfig.json
├── src/                        # Vue 3 frontend
│   ├── main.ts                 # App bootstrap
│   ├── App.vue                 # Shell + sidebar
│   ├── types/                  # TypeScript interfaces
│   ├── api/                    # Tauri invoke wrappers
│   ├── assets/main.css         # Design system (CSS vars)
│   ├── router/                 # Vue Router routes
│   ├── stores/                 # Pinia store
│   ├── components/             # Reusable components
│   └── views/                  # Page views
└── src-tauri/                  # Rust backend
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── build.rs
    └── src/
        ├── main.rs             # Binary entry point
        ├── lib.rs              # App setup + command registration
        ├── commands.rs         # IPC commands
        ├── database.rs         # Connection pool (tiberius + bb8)
        ├── queries.rs          # SQL queries
        ├── kpi.rs              # KPI calculation logic (23 tests)
        └── settings.rs         # Persistent settings (JSON)
```

## Features

### Dashboard (`/`)

- Select warehouse, year (B.E.), and month to load data
- Overall score with grade (A–D)
- DOS distribution bar (Stockout / Low / Normal / Overstock)
- Top 3 issues (Stockout, Overstock, Dead Stock)
- Full drug table with filter, search, sort, pagination

### Drug Detail (`/drug/:code`)

- 5 KPI breakdowns with individual scores
- Movement flow: Opening → Receipt → Issue → Closing
- Score breakdown with weights
- Expiry lot tracking

### Settings (`/settings`)

- SQL Server connection configuration
- Connection test
- Rolling months and expiry warning defaults

Settings are persisted as JSON at:

| OS      | Path                                  |
| ------- | ------------------------------------- |
| macOS   | `~/Library/Application Support/com.onepharm.app/` |
| Windows | `%APPDATA%\onepharm\app\data\`       |
| Linux   | `~/.local/share/inventory/`          |

## KPI System

| KPI        | Condition                  | Status            |
| ---------- | -------------------------- | ----------------- |
| DOS        | < 7 days                   | Stockout Risk     |
| DOS        | 7–14 days                  | Low Stock         |
| DOS        | 15–60 days                 | Normal            |
| DOS        | > 60 days                  | Overstock         |
| Dead Stock | RM > 0, no issue ≥ 60 days | Dead Stock        |
| Expiry     | ≤ 0 days                   | Expired           |
| Expiry     | 1–30 days                  | Critical          |
| Expiry     | 31–90 days                 | Warning           |

### Weights

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

## IPC Commands

| Command                | Description                     |
| ---------------------- | ------------------------------- |
| `health_check`         | Test database connection        |
| `get_warehouses`       | List all warehouses             |
| `get_kpi_summary`      | Warehouse-level KPI summary     |
| `get_drug_kpi_list`    | Drug list with KPIs (filtered)  |
| `get_drug_kpi_detail`  | Single drug KPI detail          |
| `get_settings`         | Retrieve current settings       |
| `save_settings`        | Save settings & reconnect       |
| `test_db_connection`   | Validate DB connection          |

## Development

### Rust

```bash
cd src-tauri
cargo fmt                          # Format
cargo clippy -- -W clippy::all     # Lint
cargo test                         # Run 23 unit tests
cargo check                        # Type-check
```

### TypeScript

```bash
npx vue-tsc --noEmit    # Type-check
npm run build           # Build (includes type-check)
```

## Security

- **Read-only** — no writes to INVS database
- Passwords stored only in local settings file
- No network exposure — direct DB connection from desktop
- Use a dedicated SQL user (not `sa`)
