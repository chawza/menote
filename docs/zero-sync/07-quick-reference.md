# Remote Sync Quick Reference — MeNote

## Architecture Summary

MeNote uses **per-user SQLite backup** sync:

- Each user has their own SQLite file (local + server backup)
- Server is "dumb" — just stores whole SQLite files
- Conflict resolution happens on the **client**
- No cross-user sync (each user is isolated)

```
┌─────────────────────────────────────────────────────────────────┐
│                     Per-User Sync Model                          │
│                                                                  │
│   Each user owns their SQLite file. No cross-user sync.         │
│   Server stores bare .sqlite files as user backups.             │
│   Conflict resolution happens on the CLIENT (each device).       │
└─────────────────────────────────────────────────────────────────┘

Device A (Phone)                          Device B (Desktop)
┌─────────────────┐                      ┌─────────────────┐
│ local SQLite    │ ◀───── sync ──────▶ │ local SQLite    │
│                 │                      │                 │
│ sync_log table  │                      │ sync_log table  │
│ - tracks changes│                      │ - tracks changes│
└─────────────────┘                      └─────────────────┘
         │                                        │
         │         ┌─────────────────────┐        │
         └────────▶│  Server (dumb storage)│◀──────┘
                   │  /backup/user123.sqlite │
                   └─────────────────────┘
```

## Sync Components

### 1. Database Tables

| Table | Purpose |
|-------|---------|
| `sync_log` | Tracks all INSERT/UPDATE/DELETE for sync |
| `sync_state` | Key-value store for sync metadata |

### 2. Tauri Commands

| Command | Purpose |
|---------|---------|
| `sync_now()` | Full sync: download, merge, upload |
| `get_sync_status()` | Returns sync state |
| `set_sync_server(url)` | Configure server URL |
| `export_sqlite()` | Export raw SQLite file |
| `import_sqlite()` | Import SQLite (new device) |

### 3. Server Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/backup/{user_id}` | GET | Download SQLite file |
| `/backup/{user_id}` | PUT | Upload SQLite file |
| `/backup/{user_id}/meta` | GET | Get file metadata |

## Conflict Resolution Rules

| Scenario | Resolution |
|----------|------------|
| Same row edited on 2 devices | Last-write-wins (compare `changed_at`) |
| Row deleted on one, edited on other | Delete wins |
| Same row inserted on 2 devices | Keep both |

## What We CAN

- ✅ Sync between multiple devices (same user)
- ✅ Self-host backup server
- ✅ User owns their SQLite file
- ✅ Deterministic conflict resolution
- ✅ Work offline, sync when online

## What We CANNOT

- ❌ Real-time sync (only on-demand)
- ❌ Cross-user collaboration
- ❌ Partial sync (whole file always)
- ❌ Background sync (app must be open)
- ❌ Web dashboard

## Implementation Checklist

- [ ] Create `sync_log` table migration
- [ ] Create `sync_state` table migration
- [ ] Wrap all mutations with `log_change()`
- [ ] Implement `sync_now()` command
- [ ] Implement `get_sync_status()` command
- [ ] Add `tauri-plugin-http` for server communication
- [ ] Create sync UI (status indicator, sync button)
- [ ] Implement first-time sync flow

## Dependencies to Add

```toml
# Cargo.toml
reqwest = { version = "0.12", features = ["json", "blocking"] }
tokio = { version = "1", features = ["full"] }
sha2 = "0.10"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
```
