# MeNote - Project Plan

## Overview
A markdown note-taking desktop app. Multi-user on one device, local-first, built with Tauri + SvelteKit + SQLite. Authentication is password-only (no username/email). Users switch by clicking profile icon.

## Tech Stack
- **Frontend**: SvelteKit + TypeScript + Vite + raw CSS (custom properties + BEM-style classes)
- **Backend**: Tauri (Rust) + SQLite + Diesel ORM
- **Bridge**: tauri-specta for TypeScript type generation from Rust types
- **Multi-user**: Multiple users on one device. Password-only auth per user (future biometrics). No email/username.
- **Design System**: "The Digital Atelier" — warm dark theme, tonal depth, gold accent, editorial typography

## Design Reference
All frontend UI follows the designs in `src-tauri/src/ui/stitch/`:
- `obsidian_gold/` — Master design system spec (colors, typography, elevation rules)
- `simple_user_login/` — Login/unlock screen
- `clean_dashboard_with_spotlight/` — Main dashboard with spotlight search
- `focused_ai_editor/` — Note editor with AI assistant panel

### Design System Principles
1. **No-Line Rule**: No explicit 1px borders. Hierarchy through tonal surface stacking only. Ghost borders at 15% opacity for accessibility only.
2. **Tonal Depth**: Elevation via progressively lighter surface colors (`surface` → `surface-container` → `surface-container-high` → `surface-container-highest`), not shadows.
3. **Warm Dark Theme**: All colors warm-tinted (no blue/purple). Gold accent `#e2c19f`.
4. **Editorial Typography**: Manrope for headings/body, Inter for labels. Large display sizes, tight tracking.
5. **Generous Whitespace**: 2-4rem padding in editors, 64px gaps between sections.
6. **Soft Corners**: Minimum 0.25rem radius. Cards use 2rem, buttons 0.75-1.5rem.
7. **AI as Whisper**: AI elements are visually secondary — gold accent hints, pulse dots, subtle status text.

---

## Phase 0: Codebase Refactoring & Foundation ✅

- [x] Global CSS with variables, utility classes, no inline styles
- [x] Biome for linting/formatting, `.editorconfig`, Zed integration
- [x] Rust backend modularized into modules with detailed error types
- [x] Frontend utilities, types, and Tauri command wrappers organized
- [x] Reusable Svelte components with consistent props pattern
- [x] App shell layout with header, sidebar, navigation components

## Phase 1: Fix Critical Bugs & Foundation
- [ ] [bug] Fix missing foreign key constraint on `notes.user_id`
- [ ] [bug] Fix `content` column allowing NULL — add NOT NULL constraint
- [ ] [data] Add migration to fix schema: foreign key, NOT NULL, CHECK(length <= 1000)
- [ ] [security] Add ownership checks on update/delete commands (verify note belongs to current user)
- [ ] [data] Add database indexes (user_id, created_at) for query performance
- [ ] [infra] Remove unused `ts-rs` dependency from Cargo.toml
- [ ] [feat] Add pagination on notes index

## Phase 2: Design System — "The Digital Atelier"

- [ ] [infra] Create global design token system in `src/lib/styles/`
  - [ ] [chore] Create `tokens.css` — CSS custom properties for all design tokens from `obsidian_gold/DESIGN.md`
  - [ ] [chore] Create `typography.css` — font-face declarations + typographic scale classes
  - [ ] [chore] Create `reset.css` — base reset + scrollbar styles
  - [ ] [chore] Create `animations.css` — keyframes (pulse, fade-in, zoom-in, underline-expand)
  - [ ] [chore] Import all stylesheets in `+layout.svelte` or `app.css`
  - [ ] [chore] Load Google Fonts (Manrope, Inter) + Material Symbols Outlined icon font
- [ ] [design] Define CSS custom properties for colors (56 named tokens)
  - [ ] Surface stack: `--color-surface` (#0f0e0d), `--color-surface-container-lowest` (#000), `--color-surface-container-low` (#141312), `--color-surface-container` (#1b1918), `--color-surface-container-high` (#211f1d), `--color-surface-container-highest` (#272523), `--color-surface-bright` (#2e2c29)
  - [ ] Primary: `--color-primary` (#e2c19f), `--color-primary-container` (#594229), `--color-on-primary` (#523c23), `--color-on-primary-container` (#eccaa8)
  - [ ] Secondary: `--color-secondary` (#a59c96), `--color-secondary-container` (#403a35), `--color-on-secondary` (#241f1b)
  - [ ] Tertiary: `--color-tertiary` (#fff8f0), `--color-on-tertiary` (#6a5e38)
  - [ ] On-surface: `--color-on-surface` (#eae4e0) — primary text (warm off-white, never pure white)
  - [ ] Outlines: `--color-outline` (#787572), `--color-outline-variant` (#4a4745)
  - [ ] Error: `--color-error` (#ed7f64), `--color-error-container` (#7e2b17)
- [ ] [design] Define typography CSS custom properties and classes
  - [ ] `--font-display`: Manrope, 3.5rem/700, letter-spacing -0.02em
  - [ ] `--font-headline`: Manrope, 1.75rem/600, letter-spacing -0.01em
  - [ ] `--font-title`: Manrope, 1rem/600
  - [ ] `--font-body`: Manrope, 1rem/400
  - [ ] `--font-label`: Inter, 0.75rem/500, uppercase, letter-spacing 0.05em
- [ ] [design] Define spacing/radius custom properties
  - [ ] `--radius-sm` 0.25rem, `--radius-md` 0.5rem, `--radius-lg` 0.75rem, `--radius-xl` 1rem, `--radius-card` 2rem, `--radius-full` 9999px
  - [ ] `--shadow-ambient`: `0px 20px 40px rgba(0, 0, 0, 0.4)` — for floating elements only
- [ ] [design] Custom scrollbar CSS: 4-6px width, transparent track, `#272523` thumb, rounded
- [ ] [feat] Create base component library (BEM-style class naming)
  - [ ] `PrimaryButton` — `primary` bg gradient (135deg), `on-primary` text, `--radius-lg`, `scale(0.98)` on active, ambient shadow
  - [ ] `TertiaryButton` — no background, `primary` text, `surface-bright` at 10% on hover
  - [ ] `GhostInput` — `surface-container-lowest` bg ("well" effect), active state: 1px ghost border at 40% `primary`
  - [ ] `AnimatedUnderlineInput` — bottom-border-only, gold underline expands from center on focus (500ms transition)
  - [ ] `TagPill` — small label with optional color, `--radius-full`
  - [ ] `AvatarCircle` — circular image with `--color-surface-container-high` fallback, grayscale→color on hover
- [ ] [chore] Remove old global CSS variables, migrate to new custom property naming
- [ ] [chore] Update existing components (Modal, ConfirmModal, Toast) to use new design tokens

## Phase 3: Multi-User Password-Only Auth

- [ ] [feat] Simplify User model — remove email, keep only `id`, `display_name`, `password_hash`, `created_at`
- [ ] [data] Create migration to alter users table (drop email unique, add password_hash if missing)
- [ ] [feat] Add password hashing with bcrypt/argon2 on Rust backend
  - [ ] [security] Add `argon2` or `bcrypt` crate dependency
  - [ ] [feat] Implement `set_password` Tauri command (hash + store)
  - [ ] [feat] Implement `verify_password` Tauri command
- [ ] [feat] Add `get_all_users` Tauri command (list all users with display_name only, no password_hash)
- [ ] [feat] Add `get_user_by_id` Tauri command
- [ ] [feat] Add `create_user` Tauri command (first-run: create user with display_name + password)
- [ ] [feat] Add `delete_user` Tauri command (cascade delete user's notes)
- [ ] [feat] App state: track active user session in Rust (in-memory, not persisted)
- [ ] [security] All note commands require active user session (return error if no session)
- [ ] [security] Add session token or user_id tracking in Rust app state
- [ ] [ux] Login/unlock screen matching `simple_user_login` design:
  - [ ] Centered layout on `surface` bg with radial gradient vignette
  - [ ] Brand block: "THE ATELIER" + "PRIVATE WORKSPACE" subtitle
  - [ ] User avatar: 96px circle, grayscale→color on hover, gold border glow
  - [ ] User display name below avatar
  - [ ] Password input: animated underline (`AnimatedUnderlineInput`), placeholder "Key required"
  - [ ] "Switch User" link — small uppercase tracked text
  - [ ] Decorative L-shaped corner elements (desktop only)
  - [ ] "End-to-End Encrypted" footer with lock icon
- [ ] [ux] User switcher: grid of user avatars with names, click to select → show password input
- [ ] [ux] First-run flow: welcome screen → create first user (display_name + password)
- [ ] [ux] "Add new user" option in user switcher
- [ ] [ux] "Delete user" option in user switcher (with confirmation, cascades to notes)
- [ ] [ux] Lock user: click lock icon or press Ctrl/Cmd+L to lock current user
- [ ] [ux] Auto-lock user after configurable idle timeout
- [ ] [desktop] Persist app window state (size, position) across sessions
- [ ] [desktop/macOS] Support Touch ID as unlock method (fallback to password)
- [ ] [desktop/windows] Support Windows Hello as unlock method
- [ ] [desktop/linux] Password-only unlock

## Phase 4: Dashboard & App Shell

- [ ] [ux] `SideNavBar` component (fixed left, w-64)
  - [ ] Brand block: gold `draw` icon in rounded gold square + "The Atelier" title + "Creative Workspace" subtitle
  - [ ] "New Note" CTA button — full-width, `primary`→`primary-container` gradient, `rounded-xl`, ambient shadow
  - [ ] Navigation links: Notes (`description`), Profile (`person`), Tags (`label`), divider, Settings (`settings`), Help (`help`)
  - [ ] Active state: `text-primary`, `bg-surface-container-high`, bold
  - [ ] Inactive state: `text-stone-400`, hover `text-stone-200` + `bg-surface-container-high`, `duration-200`
  - [ ] User profile card at bottom: circular avatar, display name, membership subtitle
- [ ] [ux] `TopNavBar` component (sticky top, z-40)
  - [ ] Left: "The Atelier" brand text in gold
  - [ ] Right: AI sparkle button (`auto_awesome` icon) + thin divider + user avatar (32px, `rounded-lg`)
  - [ ] Background: `bg-surface` with generous padding
- [ ] [ux] Main content area — greeting section
  - [ ] "Good morning, **Alex**." in `text-5xl font-bold`, name in `text-primary` (gold)
  - [ ] Subtitle: task count or context line in `text-secondary`
- [ ] [ux] Bento grid note layout (`grid-cols-12`, gap-6)
  - [ ] `NoteCardFeatured` (col-span-8): `rounded-[2rem]`, tag pill, title, preview, timestamp, action buttons ("Summarize", "Ask Agent"), arrow icon. Hover: `bg-surface-container-high`, ghost border appears
  - [ ] `NoteCardMedium` (col-span-4): icon block in `bg-secondary-container`, title, description, category footer with `more_horiz`
  - [ ] `NoteCardSmall` (col-span-4): title, description, category, indicator dot or stacked avatars
  - [ ] All cards: `surface-container-low` bg, no dividers, 1.5rem vertical spacing, hover tonal shift (no movement)
- [ ] [ux] `AIPromoCard` (col-span-4) — centered, `bg-primary-container/20`, "Unlock AI Insights" heading, sparkle icon with `hover:scale-110`, "Start Analysis" link with underline animation
- [ ] [ux] `FloatingAIDock` (fixed bottom-center)
  - [ ] Pill container: `bg-surface-container-highest/80`, `backdrop-blur-2xl`, `rounded-full`, `shadow-2xl`
  - [ ] 5 circular action buttons (48x48): text editor (`edit_note`), audio (`mic`), **main AI** (56x56, `bg-primary`, filled `auto_awesome`, `hover:scale-105 active:scale-95`), camera (`photo_camera`), file upload (`attach_file`)
  - [ ] Inactive buttons: `text-stone-400`, hover `text-on-surface` + `bg-surface-bright`
- [ ] [ux] Empty state illustration when no notes exist
- [ ] [ux] Responsive: AI panel hides below `xl` breakpoint

## Phase 5: Note Editor & Markdown

- [ ] [ux] Editor page layout (sidebar + centered editor + optional AI panel)
  - [ ] Editor area: `ml-64`, centered with `max-w-3xl`, generous left margin ("editorial gutter")
  - [ ] Status line: "Draft" dot + date, `text-secondary`, uppercase, `tracking-[0.2em]`
  - [ ] Title input: `text-5xl`, `font-extrabold`, transparent, no border, placeholder "Untitled Masterpiece"
  - [ ] Content textarea: `text-xl`, `leading-relaxed`, `min-h-[70vh]`, transparent, no border, no resize
  - [ ] Drag handle: `drag_indicator` icon, invisible by default, appears on group hover (`opacity-0` → `group-hover:opacity-100`)
  - [ ] "No chrome" aesthetic — transparent inputs, all focus via ghost borders
- [ ] [ux] AI Architect panel (right aside, `w-80`, visible at `xl` breakpoint only)
  - [ ] Container: `rounded-2xl`, `bg-surface-container`, ghost border, `backdrop-blur-xl`
  - [ ] Header: "AI Architect" with filled `smart_toy` icon (gold) + "Ready" status badge (pill, `bg-primary/10`)
  - [ ] 3 action buttons: Generate Outline (`account_tree`), Fix Grammar (`spellcheck`), Brainstorm (`psychology`)
    - [ ] Each: icon in 40x40 `rounded-lg` container (`bg-surface-container-highest`), title + description
    - [ ] Hover: `bg-surface-container-high`, icon `scale-110`
  - [ ] AI suggestion block: dark card (`bg-surface-container-lowest`), italic quote, agent avatar images + count
  - [ ] Metadata card: word count, reading time, tone (`text-primary`), gradient background
- [ ] [ux] Floating action buttons (fixed, bottom-right)
  - [ ] Agent status pill: `bg-surface-container-highest/80`, `backdrop-blur-md`, `rounded-full`, pulsing gold dot (`animate-pulse`) + "Agent listening..." text
  - [ ] Microphone FAB: 56px circle, `bg-primary`, filled `mic` icon, ambient shadow (`0px 20px 40px rgba(0,0,0,0.4)`), `hover:scale-105 active:scale-95`
- [ ] [feat] Add markdown parser library (e.g., marked, markdown-it, or unified)
- [ ] [feat] Render note content as HTML in read/view mode
- [ ] [security] Sanitize rendered HTML to prevent XSS (DOMPurify or similar)
- [ ] [ux] View mode: rendered markdown with design system typography (headings, lists, links, blockquotes, code blocks)
- [ ] [ux] Tap/click note card to view rendered, edit button to switch to raw editor
- [ ] [feat] Code syntax highlighting in rendered markdown

## Phase 6: Spotlight Search

- [ ] [feat] Add `search_notes` Tauri command (SQLite LIKE or FTS5)
- [ ] [feat] Full-text search with SQLite FTS5 (fuzzy, ranking)
- [ ] [ux] Spotlight search overlay matching `clean_dashboard_with_spotlight` design:
  - [ ] Fixed full-screen overlay (z-100), centered horizontally, pushed to `15vh` from top
  - [ ] Container: `max-w-2xl`, `bg-surface-container-highest/95`, `backdrop-blur-3xl`, `rounded-2xl`, ghost border
  - [ ] Search input: `text-xl`, placeholder "Search everything...", auto-focused
  - [ ] ESC key indicator: small pill badge "ESC" in `bg-surface-container-low`
  - [ ] Recent searches section: uppercase header, history items with `history` icon, hover `bg-primary/10`
  - [ ] Scrollable results area: `max-height 40vh`
  - [ ] Animation: fade-in + zoom-in, duration-300
- [ ] [ux] Keyboard shortcut Cmd/Ctrl+K to open spotlight
- [ ] [ux] Highlight search matches in note cards
- [ ] [ux] Filter by tag (combine with text search)
- [ ] [ux] Sort options: by date created, date updated, alphabetical
- [ ] [perf] FTS5 index for fast full-text search across all notes

## Phase 7: Tag System

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
- [ ] [ux] Display tags as colored pills/chips on note cards (using `TagPill` component)
- [ ] [ux] Tag sidebar or filter bar to filter notes by tag
- [ ] [ux] Tag autocomplete when typing `#` in note editor
- [ ] [perf] Cache extracted tags to avoid re-parsing on every render

## Phase 8: AI Features

- [ ] [feat] AI Architect backend integration
  - [ ] [feat] "Generate Outline" action — send note content to AI, return structured outline
  - [ ] [feat] "Fix Grammar" action — send note content to AI, return corrected text
  - [ ] [feat] "Brainstorm" action — send note content to AI, return expanded ideas
- [ ] [ux] AI suggestion block in editor panel — display AI-generated suggestions with agent avatars
- [ ] [ux] Agent status pill — show when AI agent is active/listening
- [ ] [ux] "Summarize" action on note cards (dashboard) — generate AI summary of note
- [ ] [ux] "Ask Agent" action on note cards (dashboard) — open chat with note context
- [ ] [feat] Microphone FAB — voice capture via Tauri, transcribe to text
- [ ] [ux] AI dock actions (from `FloatingAIDock`): text editor, audio capture, image scan, file upload
- [ ] [agent] AI-powered tag suggestions (auto-suggest tags based on note content)
- [ ] [agent] AI-powered search (natural language queries)

## Phase 9: UX Polish & Accessibility

- [x] [feat] Create `+layout.svelte` — shared app shell with header, sidebar, content area
- [x] [chore] Extract shared layout components (Header, Sidebar, Navigation) to `src/lib/components/layout/`
- [ ] [chore] Consolidate route-specific CSS to route-scoped styles
- [ ] [chore] Extract animation/transition utilities to `src/lib/utils/transitions.ts`
- [ ] [chore] Extract keyboard shortcut handler to `src/lib/utils/keyboard.ts`
- [ ] [ux] Add loading spinner/skeleton for initial data fetch
- [ ] [ux] Smooth transitions between note list, view, and edit states
- [ ] [ux] Keyboard shortcuts (Ctrl+N new note, Ctrl+S save, Cmd/Ctrl+K search, Cmd+/ toggle sidebar)
- [ ] [ux] Confirm before navigating away from unsaved edits
- [ ] [a11y] Add ARIA labels to interactive elements
- [ ] [a11y] Ensure keyboard navigation works for all actions
- [ ] [a11y] Proper focus management (modal trap, return focus after close)
- [ ] [a11y] Color contrast meets WCAG AA for dark and light themes
- [ ] [a11y] Screen reader announcements for toast notifications

## Phase 10: Data Safety & Export

- [ ] [feat] Add `export_notes` Tauri command (export as .json or .md zip)
- [ ] [feat] Add `import_notes` Tauri command (import from json backup)
- [ ] [feat] Database backup: auto-backup SQLite file on app update
- [ ] [ux] Settings page with export/import buttons
- [ ] [security] Encrypt backup file with user password
- [ ] [desktop/macOS] Store database in `~/Library/Application Support/MeNote/`
- [ ] [desktop/windows] Store database in `%APPDATA%/MeNote/`
- [ ] [desktop/linux] Store database in `~/.local/share/MeNote/`

## Phase 11: Security Hardening

- [ ] [security] Enable Content Security Policy in tauri.conf.json
- [ ] [security] Input validation on all Tauri commands (content length, required fields)
- [ ] [security] Rate limit password attempts (prevent brute force)
- [ ] [security] Secure password storage: verify argon2/bcrypt is used correctly
- [ ] [security] Audit Tauri capabilities — minimize permissions in capabilities/default.json
- [ ] [security] No logging of sensitive data (passwords, note content in debug)

## Phase 12: Testing

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
  - [ ] [feat] E2E test: create user, switch user, lock/unlock user flow
- [ ] [infra] CI pipeline (GitHub Actions)
  - [ ] [feat] Run `cargo clippy` + `cargo fmt --check` + `cargo test`
  - [ ] [feat] Run `yarn check` (Svelte type checking)
  - [ ] [feat] Run `yarn lint` if ESLint is configured

## Phase 13: Build & Distribution

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

## Phase 14: Performance & Polish

- [ ] [chore] Optimize bundle size (code splitting, lazy loading routes)
- [ ] [chore] Audit and remove unused dependencies
- [ ] [chore] Consolidate duplicate utility functions across the codebase
- [ ] [chore] Extract theme logic to dedicated `src/lib/stores/theme.ts` store
- [ ] [chore] Extract user session logic to `src/lib/stores/session.ts` store
- [ ] [perf] Database connection pooling (r2d2-diesel or similar)
- [ ] [perf] Virtual scrolling for large note lists
- [ ] [perf] Lazy load note content (paginate get_notes)
- [ ] [perf] Debounce tag extraction on note save
- [ ] [perf] Memoize expensive computations in Svelte components
- [ ] [ux] Dark/light theme toggle (persist preference)
- [ ] [ux] Custom app title bar (frameless window)
- [ ] [ux] System tray support (quick note creation)
- [ ] [desktop/macOS] Native menu bar (File, Edit, View, Help)
- [ ] [desktop/windows] Native menu bar integration
- [ ] [ux] Note character counter while editing
- [ ] [ux] Drag-to-resize sidebar

## Future (Not Planned Now)

- [feat] Cloud sync
- [feat] Note attachments (images, files)
- [feat] Note categories/folders
- [feat] Collaborative editing
- [design] Light theme (all current designs are dark-only)
- [design] Settings page (nav link exists, no mockup)
- [design] Profile page (nav link exists, no mockup)
- [design] Tags management page (nav link exists, no mockup)
- [design] Help page (nav link exists, no mockup)
- [design] Multi-user picker grid (login shows one user + "Switch User" only)
- [design] Toast/notification visuals (no mockup in stitch designs)
- [design] Confirm modal visuals (no mockup in stitch designs)
