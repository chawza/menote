# System Architecture

This document describes the architecture and design of the MeNote application.

## Overview

MeNote follows a **desktop-first architecture** using Tauri, which combines a modern web frontend with a native Rust backend. This approach provides:

- **Native Performance**: Rust backend for data processing and storage
- **Modern UI**: SvelteKit frontend with reactive components
- **Cross-Platform**: Single codebase for Windows, macOS, and Linux
- **Small Bundle Size**: Compared to Electron-based apps
- **Security**: Tauri's security features and Rust's memory safety

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     User Interface                          │
│              (SvelteKit + TypeScript + Vite)               │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Routes    │  │ Components  │  │    Stores   │         │
│  │  (+page.*)  │  │  (.svelte)  │  │  (state)    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Tauri Bridge
                     │ (invoke/callback API)
                     │
┌────────────────────▼────────────────────────────────────────┐
│                    Rust Backend                             │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Commands  │  │    Models   │  │  Database   │         │
│  │  (lib.rs)   │  │  (structs)  │  │   (Diesel)  │         │
│  └─────────────┘  └─────────────┘  └──────┬──────┘         │
└───────────────────────────────────────────┼─────────────────┘
                                            │
                                   ┌────────▼────────┐
                                   │     SQLite      │
                                   │  (menote.db)    │
                                   └─────────────────┘
```

## Frontend Architecture

### SvelteKit Structure

The frontend is built with **SvelteKit 2.x** using the following patterns:

```
src/
├── lib/
│   ├── components/        # Reusable UI components
│   ├── stores/           # Svelte stores for state management
│   └── utils/            # Utility functions
├── routes/
│   ├── +page.svelte      # Home page
│   ├── +layout.svelte    # Root layout
│   └── ...               # Other routes
└── app.html              # HTML template
```

### State Management

- **Svelte Stores**: For local component state
- **Tauri Events**: For backend-to-frontend communication
- **Invoke API**: For frontend-to-backend commands

### Frontend-Backend Communication

```typescript
// Example: Calling a Rust command from Svelte
import { invoke } from '@tauri-apps/api/core';

async function createNote(content: string) {
  const note = await invoke('create_note', { content });
  return note;
}
```

## Backend Architecture

### Rust Structure

```
src-tauri/src/
├── main.rs               # Application entry point
└── lib.rs                # Commands and app logic
```

### Commands

Tauri commands are Rust functions exposed to the frontend:

```rust
#[tauri::command]
async fn create_note(content: String, state: State<'_, AppState>) -> Result<Note, Error> {
    // Create note in database
    let note = state.db.create_note(&content).await?;
    Ok(note)
}
```

### Type Safety with ts-rs

Rust structs automatically generate TypeScript types:

```rust
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
```

Generates:

```typescript
interface Note {
  id: number;
  content: string;
  created_at: string;
}
```

## Database Architecture

### SQLite with Diesel ORM

- **Database**: SQLite (embedded, no external server)
- **ORM**: Diesel for type-safe SQL queries
- **Migrations**: Version-controlled schema changes

### Database Schema

See [models.md](./models.md) for detailed model definitions.

### Migration Workflow

```bash
# Create new migration
diesel migration generate create_notes_table

# Edit migration files (up.sql and down.sql)

# Run migrations
diesel migration run

# Generate Rust schema
diesel print-schema > src/schema.rs
```

## Security Architecture

### Tauri Security Features

1. **Content Security Policy (CSP)**: Configured in `tauri.conf.json`
2. **Capability-based Access**: APIs are explicitly allowed
3. **Process Isolation**: Webview is sandboxed from system
4. **Secure Commands**: All backend functions are explicitly exposed

### Data Security

- **Local Storage**: All data stored locally in SQLite
- **No Network**: Application works offline
- **User Authentication**: Multi-user support with password hashing
- **Input Validation**: All inputs validated on backend

## Performance Considerations

### Frontend

- **Code Splitting**: Automatic with SvelteKit
- **Lazy Loading**: Routes loaded on demand
- **Reactive Updates**: Svelte 5 runes for efficient reactivity

### Backend

- **Async/Await**: Non-blocking I/O operations
- **Connection Pooling**: Database connection reuse
- **Compiled Binary**: Rust produces optimized native code

### Bundle Size

- **Minimal Runtime**: Tauri runtime is ~600KB
- **Tree Shaking**: Unused code eliminated
- **Asset Optimization**: Images and fonts optimized

## Component Interaction

### User Flow Example: Creating a Note

```
1. User clicks "New Note" button
   └── Frontend: Update UI state

2. User types note content
   └── Frontend: Local state updates (reactive)

3. User clicks "Save"
   └── Frontend: invoke('create_note', { content })
   
4. Tauri Bridge
   └── Serializes request, calls Rust function

5. Rust Backend
   └── Validates input
   └── Executes Diesel query
   └── Returns Note struct

6. Tauri Bridge
   └── Deserializes response

7. Frontend
   └── Updates store with new note
   └── UI re-renders automatically
```

## Technology Stack Summary

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Frontend Framework | SvelteKit 2.x | UI components, routing |
| Frontend Language | TypeScript | Type safety |
| Build Tool | Vite | Fast builds, HMR |
| Desktop Framework | Tauri 2.x | Native window, bridge |
| Backend Language | Rust | Performance, safety |
| Database | SQLite | Embedded storage |
| ORM | Diesel | Type-safe queries |
| Type Bridge | ts-rs | Rust ↔ TypeScript types |
| Package Manager | Yarn | Node dependencies |

## Future Considerations

Potential architectural enhancements:

1. **Synchronization**: Cloud sync with conflict resolution
2. **Plugins**: Extension system for custom features
3. **Theming**: Custom CSS themes support
4. **Search**: Full-text search with SQLite FTS
5. **Export/Import**: Various format support (Markdown, PDF, etc.)

## References

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Diesel Documentation](https://diesel.rs/)
- [ts-rs Documentation](https://github.com/Aleph-Alpha/ts-rs)
