# MeNote - Project Plan

## Overview
A markdown note-taking desktop app. Single-user, local-first, built with Tauri + SvelteKit + SQLite.

## Tech Stack
- **Frontend**: SvelteKit + TypeScript + Vite
- **Backend**: Tauri (Rust) + SQLite + Diesel ORM
- **Bridge**: tauri-specta for TypeScript type generation from Rust types
- **Single user**: One user at a time. Simple password lock (no email, no signup).

---

## Phase 1: Fix Critical Bugs & Foundation

- [ ] [bug] Fix `update_note` — adds `.filter(id.eq(note.id))` to prevent updating ALL rows
- [ ] [bug] Fix missing foreign key constraint on `notes.user_id`
- [ ] [bug] Fix `content` column allowing NULL — add NOT NULL constraint
- [ ] [data] Add migration to fix schema: foreign key, NOT NULL, CHECK(length <= 1000)
- [ ] [security] Add ownership checks on update/delete commands (verify note belongs to current user)
- [ ] [data] Add database indexes (user_id, created_at) for query performance
- [ ] [infra] Remove unused `ts-rs` dependency from Cargo.toml
- [ ] [bug] Fix `get_all_users` command — should not exist (single user app)

## Phase 2: Simplify to Single User + Password Lock

- [ ] [feat] Simplify User model — remove email, keep only `id`, `display_name`, `password_hash`, `created_at`
- [ ] [data] Create migration to alter users table (drop email unique, add password_hash if missing)
- [ ] [feat] Add password hashing with bcrypt/argon2 on Rust backend
  - [ ] [security] Add `argon2` or `bcrypt` crate dependency
  - [ ] [feat] Implement `set_password` Tauri command (hash + store)
  - [ ] [feat] Implement `verify_password` Tauri command
- [ ] [feat] Add `get_current_user` Tauri command
- [ ] [feat] Add `setup_user` Tauri command (first-run: create user with display_name + password)
- [ ] [feat] App state: track if user is unlocked in Rust (in-memory, not persisted)
- [ ] [security] All note commands require unlocked session (return error if locked)
- [ ] [ux] First-run flow: welcome screen → set display name → set password
- [ ] [ux] Lock screen: password input to unlock app
- [ ] [ux] Auto-lock after configurable idle timeout
- [ ] [ux] "Lock app" button in app header
- [ ] [desktop] Persist app window state (size, position) across sessions
- [ ] [desktop/macOS] Support Touch ID as unlock method (fallback to password)
- [ ] [desktop/windows] Support Windows Hello as unlock method
- [ ] [desktop/linux] Password-only unlock

## Phase 3: Tag System

- [ ] [data] Create `tags` table migration (id, name, color, created_at)
- [ ] [data] Create `note_tags` join table migration (note_id, tag_id, created_at)
- [ ] [data] Add indexes on note_tags (note_id, tag_id)
- [ ] [feat] Add Tag + NoteTag Diesel models
- [ ] [feat] Implement tag extraction from note content (regex: `#([a-zA-Z0-9_-]+)`)
- [ ] [feat] Auto-create tags on note save (upsert tag names, update note_tag relations)
- [ ] [feat] Remove stale tag relations when note content changes
- [ ] [feat] Add `get_all_tags` Tauri command
- [ ] [feat] Add `get_notes_by_tag` Tauri command
- [ ] [feat] Add tag color support (optional hex color per tag)
- [ ] [ux] Display tags as colored pills/chips on note cards
- [ ] [ux] Tag sidebar or filter bar to filter notes by tag
- [ ] [ux] Tag autocomplete when typing `#` in note editor
- [ ] [perf] Cache extracted tags to avoid re-parsing on every render

## Phase 4: Markdown Rendering

- [ ] [feat] Add markdown parser library (e.g., marked, markdown-it, or unified)
- [ ] [feat] Render note content as HTML in read/view mode
- [ ] [security] Sanitize rendered HTML to prevent XSS (DOMPurify or similar)
- [ ] [ux] Edit mode: raw textarea (current behavior)
- [ ] [ux] View mode: rendered markdown with nice typography
- [ ] [ux] Tap/click note card to view rendered, edit button to switch to raw
- [ ] [feat] Support code syntax highlighting in rendered markdown
- [ ] [ux] Style rendered markdown (headings, lists, links, blockquotes, code blocks)

## Phase 5: UX Polish & App Shell

- [ ] [feat] Create `+layout.svelte` — shared app shell with header, sidebar, content area
- [ ] [ux] Move global CSS variables to `+layout.svelte` or `app.css`
- [ ] [ux] Add loading spinner/skeleton for initial data fetch
- [ ] [ux] Add empty state illustration when no notes exist
- [ ] [ux] Smooth transitions between note list, view, and edit states
- [ ] [ux] Keyboard shortcuts (Ctrl+N new note, Ctrl+S save, Ctrl+F search, Escape close)
- [ ] [ux] Confirm before navigating away from unsaved edits
- [ ] [a11y] Add ARIA labels to interactive elements
- [ ] [a11y] Ensure keyboard navigation works for all actions
- [ ] [a11y] Proper focus management (modal trap, return focus after close)
- [ ] [a11y] Color contrast meets WCAG AA for dark and light themes
- [ ] [a11y] Screen reader announcements for toast notifications

## Phase 6: Search & Filtering

- [ ] [feat] Add `search_notes` Tauri command (SQLite LIKE or FTS5)
- [ ] [feat] Full-text search with SQLite FTS5 (fuzzy, ranking)
- [ ] [ux] Search bar in app header with debounce
- [ ] [ux] Highlight search matches in note cards
- [ ] [ux] Filter by tag (combine with text search)
- [ ] [ux] Sort options: by date created, date updated, alphabetical
- [ ] [perf] FTS5 index for fast full-text search across all notes

## Phase 7: Data Safety & Export

- [ ] [feat] Add `export_notes` Tauri command (export as .json or .md zip)
- [ ] [feat] Add `import_notes` Tauri command (import from json backup)
- [ ] [feat] Database backup: auto-backup SQLite file on app update
- [ ] [ux] Settings page with export/import buttons
- [ ] [security] Encrypt backup file with user password
- [ ] [desktop/macOS] Store database in `~/Library/Application Support/MeNote/`
- [ ] [desktop/windows] Store database in `%APPDATA%/MeNote/`
- [ ] [desktop/linux] Store database in `~/.local/share/MeNote/`

## Phase 8: Security Hardening

- [ ] [security] Enable Content Security Policy in tauri.conf.json
- [ ] [security] Input validation on all Tauri commands (content length, required fields)
- [ ] [security] Rate limit password attempts (prevent brute force)
- [ ] [security] Secure password storage: verify argon2/bcrypt is used correctly
- [ ] [security] Audit Tauri capabilities — minimize permissions in capabilities/default.json
- [ ] [security] No logging of sensitive data (passwords, note content in debug)

## Phase 9: Testing

- [ ] [infra] Add Rust test framework (cargo test)
  - [ ] [feat] Unit tests for tag extraction (regex parsing)
  - [ ] [feat] Unit tests for password hashing/verification
  - [ ] [feat] Unit tests for note CRUD operations
  - [ ] [feat] Unit tests for input validation
  - [ ] [feat] Integration tests for Tauri commands
- [ ] [infra] Add frontend test framework (vitest + @testing-library/svelte)
  - [ ] [feat] Component tests for NoteCard, Modal, Toast
  - [ ] [feat] Store tests for toast store
  - [ ] [feat] Utility tests for tryCommand wrapper
- [ ] [infra] Add E2E test framework (Playwright or Tauri WebDriver)
  - [ ] [feat] E2E test: create, edit, delete note flow
  - [ ] [feat] E2E test: lock/unlock flow
- [ ] [infra] CI pipeline (GitHub Actions)
  - [ ] [feat] Run `cargo clippy` + `cargo fmt --check` + `cargo test`
  - [ ] [feat] Run `yarn check` (Svelte type checking)
  - [ ] [feat] Run `yarn lint` if ESLint is configured

## Phase 10: Build & Distribution

- [ ] [infra] Configure app metadata (name, version, identifier) in tauri.conf.json
- [ ] [infra] App icon set (custom, not default Tauri icon)
- [ ] [desktop/macOS] Build .dmg installer with code signing
- [ ] [desktop/macOS] Notarize app for macOS Gatekeeper
- [ ] [desktop/windows] Build .msi installer
- [ ] [desktop/windows] Code signing with Windows certificate
- [ ] [desktop/linux] Build .deb and .AppImage
- [ ] [infra] Automated release pipeline (tag → build → upload artifacts)
- [ ] [desktop/macOS] Support for Apple Silicon (aarch64) + Intel (x86_64)
- [ ] [desktop] Auto-updater (Tauri updater plugin)

## Phase 11: Performance & Polish

- [ ] [perf] Database connection pooling (r2d2-diesel or similar)
- [ ] [perf] Virtual scrolling for large note lists
- [ ] [perf] Lazy load note content (paginate get_notes)
- [ ] [perf] Debounce tag extraction on note save
- [ ] [ux] Dark/light theme toggle (persist preference)
- [ ] [ux] Custom app title bar (frameless window)
- [ ] [ux] System tray support (quick note creation)
- [ ] [desktop/macOS] Native menu bar (File, Edit, View, Help)
- [ ] [desktop/windows] Native menu bar integration
- [ ] [ux] Note character counter while editing
- [ ] [ux] Drag-to-resize sidebar

## Future (Not Planned Now)

- [agent] AI-powered note summarization
- [agent] AI-powered tag suggestions
- [agent] AI-powered search (natural language)
- [feat] Cloud sync
- [feat] Note attachments (images, files)
- [feat] Note categories/folders
- [feat] Collaborative editing
