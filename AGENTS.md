# MeNote Agent Guidelines

This file contains essential information for AI agents working on the MeNote project.

## Project Overview

MeNote is a markdown note-taking application built with:
- **Frontend**: SvelteKit + TypeScript
- **Backend**: Tauri (Rust) + SQLite + Diesel ORM
- **Bridge**: ts-rs for TypeScript type generation from Rust types

## Tech Stack

- **Frontend Framework**: SvelteKit 2.x with Svelte 5
- **Desktop Framework**: Tauri 2.x
- **Database**: SQLite with Diesel ORM
- **Build Tool**: Vite
- **Package Manager**: Yarn

## Project Structure

```
menote/
├── src/                    # SvelteKit frontend source
│   ├── lib/               # Shared components and utilities
│   ├── routes/            # SvelteKit routes
│   └── app.html           # HTML template
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/              # Rust source code
│   │   ├── main.rs       # Entry point
│   │   └── lib.rs        # Library and commands
│   ├── migrations/       # Diesel database migrations
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── docs/                  # Documentation
├── static/               # Static assets
├── build/                # Production build output
└── package.json          # Node.js dependencies
```

## Key Files to Reference

- `docs/PLAN.md` - Project requirements and specifications
- `docs/dev-guide.md` - How to run the project
- `docs/build-desktop.md` - Desktop build instructions
- `docs/architecture.md` - System architecture
- `docs/models.md` - Data models documentation

## Development Commands

Always run these from the project root:

```bash
# Start development server
yarn tauri dev

# Build frontend only
yarn build

# Type checking
yarn check

# Build desktop app
yarn tauri build
```

## Code Style Guidelines

### TypeScript/Svelte
- Use TypeScript strictly (no `any` types)
- Follow SvelteKit conventions
- Use kebab-case for file names
- Use PascalCase for component names

### Rust
- Follow Rust naming conventions (snake_case)
- Use proper error handling with `Result` types
- Document public APIs with doc comments
- Use ts-rs derive macros for types exposed to frontend

## Important Conventions

1. **Type Safety**: Always use ts-rs to generate TypeScript types from Rust structs. Never manually maintain type definitions in both places.

2. **Database**: Use Diesel migrations for all schema changes. Never modify schema directly.

3. **Commands**: All Tauri commands should be defined in `src-tauri/src/lib.rs` and registered in the `run()` function.

4. **Frontend-Backend Communication**: Use Tauri's invoke API for all frontend-to-backend communication.

## Lint and Type Check Commands

Before completing any task, run:

```bash
# TypeScript/Svelte type checking
yarn check

# Rust linting (if applicable)
cd src-tauri && cargo clippy

# Rust formatting check
cd src-tauri && cargo fmt --check
```

## Common Tasks

### Adding a new Tauri command

1. Define the function in `src-tauri/src/lib.rs` with `#[tauri::command]` attribute
2. Add the function to `tauri::generate_handler![]` macro
3. Generate TypeScript bindings if the function returns custom types
4. Call from frontend using `invoke('command_name', { args })`

### Adding a database model

1. Create migration: `diesel migration generate migration_name`
2. Define schema in migration files
3. Run migration: `diesel migration run`
4. Define model struct in Rust with Diesel derive macros
5. Add TS derive macro for type generation
6. Generate TypeScript types

### Creating new routes

1. Add route in `src/routes/` following SvelteKit file-based routing
2. Create `+page.svelte` for the route component
3. Create `+page.ts` or `+page.server.ts` for data loading if needed

## Testing

- Write unit tests for Rust code in `src-tauri/src/`
- Use `cargo test` to run Rust tests
- Test frontend components manually via `yarn tauri dev`

## Security Notes

- Never commit database files or credentials
- Never expose sensitive data through Tauri commands
- Use Tauri's security features (CSP, capabilities) properly
- All database queries should be parameterized (Diesel handles this)

## Questions?

If unclear about requirements or implementation details, ask for clarification before proceeding.
