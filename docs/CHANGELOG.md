# Changelog

## Phase 0: Codebase Refactoring & Foundation — Complete

### Rust Backend Modularization

- Extracted `db.rs` from `lib.rs` — `establish_connection()` and `setup_data_base()` now live in their own module
- Split `models.rs` into `models/` directory with per-entity files (`notes.rs`, `users.rs`), each combining Diesel model + DTOs
- Split `error.rs` into `error/mod.rs` with `AppError` helper constructors (`note_not_found()`, `missing_id()`, `validation()`, `unauthorized()`) and `From` impls
- Extracted user commands to `commands/users.rs` with `commands/mod.rs` for module re-exports
- Notes CRUD commands remain in `lib.rs`; user commands extracted to `commands/users.rs`

### Frontend Organization

- Created `src/lib/commands/index.ts` — re-exports commands from auto-generated bindings
- Created `src/lib/types/index.ts` — re-exports types from bindings + local toast types
- Created `src/lib/utils/date.ts` — `formatLocal()` and `nowUnix()` date utilities
- Refactored `src/lib/utils/tauri.ts` — `tryCommand()` wrapper with typed `CommandResult<T>` and toast error handling

### Components & Storybook

- Added Storybook 10 with `@storybook/sveltekit` + `@storybook/addon-svelte-csf`
- Created stories for every component (`Button`, `Card`, `Modal`, `ConfirmModal`, `Header`, `Sidebar`, `TextArea`, `Toast`, `ToastContainer`)
- New components: `Button`, `Card`, `Header`, `Sidebar`, `TextArea`
- Updated existing components: `Modal`, `ConfirmModal`, `Toast`, `ToastContainer` (Svelte 5 runes, consistent props)

### App Shell & Routing

- Moved app to `src/routes/notes/` with app shell layout (Header + Sidebar)
- Created `src/routes/+layout.svelte` — root layout with global CSS + `ToastContainer`
- Created `src/routes/notes/+layout.svelte` — app shell with Header, Sidebar, content area

### Styling

- Created `src/app.css` — global CSS with design tokens (light/dark theme), utility classes (`.btn`, `.card`, `.input`, `.overlay`, `.app-shell`)
- Removed inline styles from `+page.svelte`

### Tooling & Config

- Added Biome for linting/formatting with `.editorconfig` and Zed integration
- Added `biome.json` configuration
- Added `vitest.shims.d.ts`

### Design References

- Added stitch design mockups in `src-tauri/src/ui/stitch/`: `obsidian_gold`, `simple_user_login`, `clean_dashboard_with_spotlight`, `focused_ai_editor`
- Added `DEV_HANDBOOK/making_storeis.md` — guide for writing Storybook stories
