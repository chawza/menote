# Development Guide

This guide explains how to set up and run the MeNote project for development.

## Prerequisites

Before you start, ensure you have the following installed:

### Required

- **Node.js** (v18 or higher) - [Download](https://nodejs.org/)
- **Yarn** - Install with: `npm install -g yarn`
- **Rust** - [Install](https://www.rust-lang.org/tools/install)
- **Tauri CLI** - Install with: `cargo install tauri-cli`

### Optional

- **Diesel CLI** (for database migrations) - Install with: `cargo install diesel_cli --no-default-features --features sqlite`

## Initial Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd menote
   ```

2. **Install Node.js dependencies**
   ```bash
   yarn install
   ```

3. **Install Rust dependencies** (this happens automatically on first build)

4. **Set up the database** (when database features are implemented)
   ```bash
   cd src-tauri
   diesel setup
   diesel migration run
   cd ..
   ```

## Running the Development Server

The standard way to run the project during development:

```bash
yarn tauri dev
```

This command:
1. Starts the Vite development server for the frontend
2. Compiles the Rust backend
3. Launches the Tauri desktop application
4. Enables hot-reload for both frontend and backend changes

### Development URLs

- **Frontend Dev Server**: http://localhost:1420
- **Tauri WebView**: Runs as native desktop window

## Available Scripts

From the project root, you can run:

| Command | Description |
|---------|-------------|
| `yarn dev` | Start frontend dev server only (without Tauri) |
| `yarn build` | Build frontend for production |
| `yarn preview` | Preview production build |
| `yarn check` | Run TypeScript and Svelte type checking |
| `yarn check:watch` | Run type checking in watch mode |
| `yarn tauri dev` | Start full Tauri development environment |
| `yarn tauri build` | Build production desktop application |

## Development Workflow

1. **Start the dev server**
   ```bash
   yarn tauri dev
   ```

2. **Make changes** to either:
   - Frontend files in `src/` (SvelteKit components, routes, etc.)
   - Backend files in `src-tauri/src/` (Rust code)

3. **Hot reload** will automatically update the application

4. **Run type checking** before committing:
   ```bash
   yarn check
   ```

5. **Run Rust linting** (if you modified Rust code):
   ```bash
   cd src-tauri && cargo clippy
   ```

## Project Structure During Development

```
menote/
├── src/                    # Frontend source (SvelteKit)
│   ├── lib/
│   ├── routes/
│   ├── app.html
│   └── ...
├── src-tauri/             # Backend source (Rust)
│   ├── src/
│   │   ├── lib.rs        # Commands and app setup
│   │   └── main.rs       # Entry point
│   ├── Cargo.toml
│   └── tauri.conf.json
├── build/                 # Generated frontend build
└── node_modules/          # Node dependencies
```

## Troubleshooting

### Port Already in Use

If port 1420 is already in use, Tauri will automatically try the next available port.

### Rust Compilation Errors

Ensure you have the latest Rust version:
```bash
rustup update
```

### Node Module Issues

If you encounter issues with dependencies:
```bash
rm -rf node_modules
yarn install
```

### Database Issues

If the database is not working:
1. Ensure Diesel CLI is installed
2. Check that SQLite is available on your system
3. Run migrations: `cd src-tauri && diesel migration run`

## IDE Recommendations

For the best development experience, use:

- **VS Code** with extensions:
  - [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Next Steps

- Read [architecture.md](./architecture.md) for system design details
- Read [models.md](./models.md) for data model specifications
- Read [build-desktop.md](./build-desktop.md) for creating production builds
