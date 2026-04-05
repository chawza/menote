# MeNote

A local-first markdown note-taking desktop app. Multi-user on one device, password-only auth, no cloud required.

## About

MeNote is a desktop note-taking application designed for simplicity and privacy. Multiple users can share one device, each with their own password-protected space. Notes are stored locally in SQLite — no accounts, no cloud, no tracking.

Key features:

- **Local-first** — all data stays on your machine in a SQLite database
- **Multi-user** — multiple profiles on one device, switch by clicking the profile icon
- **Password-only auth** — no email, no username, just a password to unlock your notes
- **Markdown notes** — write in markdown, rendered beautifully
- **Dark theme** — warm dark design with gold accents and editorial typography

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | SvelteKit 2 + Svelte 5 + TypeScript + Vite |
| Desktop | Tauri 2 (Rust) |
| Database | SQLite via Diesel ORM with embedded migrations |
| Type Bridge | tauri-specta + specta (auto-generated TypeScript types from Rust) |
| Component Viewer | Storybook 10 |
| Linting | Biome (JS/TS/Svelte), Clippy + rustfmt (Rust) |
| Package Manager | Yarn |

## Getting Started

### Prerequisites

- **Node.js** v18+
- **Yarn** — `npm install -g yarn`
- **Rust** — [rustup.rs](https://rustup.rs)
- **Tauri CLI** — `cargo install tauri-cli`
- **Diesel CLI** (optional, for migrations) — `cargo install diesel_cli --no-default-features --features sqlite`

### Setup

```bash
git clone <repository-url>
cd menote
yarn install
```

### Run

```bash
yarn tauri dev        # Full app (frontend + Rust backend)
yarn dev              # Frontend dev server only (no Tauri)
```

## Commands

### Frontend

| Command | Description |
|---------|-------------|
| `yarn dev` | Start frontend dev server |
| `yarn build` | Production build |
| `yarn check` | TypeScript + Svelte type checking |
| `yarn check:watch` | Type checking in watch mode |
| `yarn lint` | Biome lint + format check |
| `yarn lint:fix` | Biome lint + format fix |
| `yarn format` | Biome format (write) |
| `yarn storybook` | Start Storybook on port 6006 |
| `yarn build-storybook` | Build static Storybook site |

### Desktop

| Command | Description |
|---------|-------------|
| `yarn tauri dev` | Start full dev environment |
| `yarn tauri build` | Build production desktop app |

### Rust

| Command | Description |
|---------|-------------|
| `cd src-tauri && cargo clippy` | Lint Rust code |
| `cd src-tauri && cargo fmt --check` | Check Rust formatting |
| `cd src-tauri && cargo test` | Run Rust tests |

## Project Structure

```
menote/
├── src/                          # SvelteKit frontend
│   ├── app.html                  # HTML template
│   ├── app.css                   # Global CSS (design tokens, themes)
│   ├── bindings.ts               # Auto-generated TS types (DO NOT EDIT)
│   └── lib/
│       ├── components/           # Reusable Svelte components + Storybook stories
│       ├── commands/             # Re-exports Tauri commands
│       ├── stores/               # Svelte stores (e.g. toast notifications)
│       ├── types/                # TypeScript interfaces
│       └── utils/                # Helpers (tryCommand, date utils)
├── src-tauri/                    # Tauri Rust backend
│   ├── src/
│   │   ├── lib.rs                # App setup + note CRUD commands
│   │   ├── db.rs                 # SQLite connection + migration runner
│   │   ├── commands/             # Tauri command handlers (users, etc.)
│   │   ├── models/               # Diesel models + DTOs (notes, users)
│   │   ├── error/                # AppError enum
│   │   └── schema.rs             # Auto-generated DB schema (DO NOT EDIT)
│   └── migrations/               # Diesel SQL migrations
├── docs/                         # Documentation
│   ├── PLAN.md                   # Project roadmap and specs
│   ├── architecture.md           # System architecture
│   ├── models.md                 # Data models
│   └── dev-guide.md              # Detailed dev setup guide
└── static/                       # Static assets
```

## Architecture

```
┌─────────────────────┐       invoke()        ┌──────────────────────┐
│   SvelteKit 5 UI    │ ─────────────────────▶ │   Tauri 2 (Rust)     │
│   TypeScript + Vite │ ◀───────────────────── │   Commands            │
│                     │   Result<T, AppError>  │                      │
└─────────────────────┘                        │   ┌──────────────┐   │
                                               │   │ Diesel ORM   │   │
                                               │   │      │       │   │
                                               │   │  SQLite DB   │   │
                                               │   └──────────────┘   │
                                               └──────────────────────┘
```

- Frontend calls backend via `tryCommand(() => commands.xxx())` — a thin wrapper that handles errors and shows toast notifications
- All Tauri commands return `Result<T, AppError>` serialized as `{ status: "ok", data } | { status: "error", error }`
- TypeScript bindings are auto-generated in debug builds via tauri-specta → `src/bindings.ts`
- SQLite database lives in the Tauri app data directory, migrations run on startup
- Frontend uses Svelte 5 runes (`$state`, `$props`, `$effect`, `$derived`) and snippet blocks

## Contributing

See [`docs/dev-guide.md`](docs/dev-guide.md) for detailed setup instructions and [`docs/PLAN.md`](docs/PLAN.md) for the project roadmap and current phase.

When contributing:

- Run `yarn check` and `yarn lint` after frontend changes
- Run `cd src-tauri && cargo clippy` after Rust changes
- Every new Svelte component needs a corresponding `*.stories.svelte` file
- Use Diesel migrations for all schema changes — never edit `schema.rs` directly
