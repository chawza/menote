# Requirements for Remote Sync

## Overview

MeNote's remote sync requires:
1. **Local**: `sync_log` table to track changes
2. **Server**: Simple file storage for SQLite backups
3. **Network**: `tauri-plugin-http` for HTTPS requests

## Rust Dependencies

```toml
# src-tauri/Cargo.toml
[dependencies]
# Already present:
tauri = "2"
serde = { version = "1", features = ["derive"] }
diesel = { version = "2", features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }

# NEW for sync:
reqwest = { version = "0.12", features = ["json", "blocking"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
sha2 = "0.10"  # For file hashing
```

## New Database Schema

### sync_log Table

```sql
CREATE TABLE sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    row_id TEXT NOT NULL,
    operation TEXT NOT NULL,  -- INSERT, UPDATE, DELETE
    changed_at TEXT NOT NULL, -- ISO8601 timestamp
    old_values TEXT,          -- JSON of previous state (for UPDATE/DELETE)
    new_values TEXT,          -- JSON of new state (for INSERT/UPDATE)
    synced INTEGER DEFAULT 0  -- boolean
);

-- Indexes for fast queries
CREATE INDEX idx_sync_log_synced ON sync_log(synced);
CREATE INDEX idx_sync_log_table_row ON sync_log(table_name, row_id);
```

### sync_state Table

```sql
CREATE TABLE sync_state (
    key TEXT PRIMARY KEY,
    value TEXT
);

-- Stored values:
-- last_sync_time: ISO8601 timestamp of last successful sync
-- device_id: UUID of this device
-- device_name: human-readable device name
-- server_url: URL of backup server
```

## Server Requirements

| Component | Requirement |
|-----------|-------------|
| Type | File storage only (no sync logic) |
| Storage | SQLite files, one per user |
| Protocol | HTTPS |
| Auth | Token-based (user provides) |
| Endpoints | `GET/PUT /backup/{user_id}`, `GET /backup/{user_id}/meta` |

## Minimal Sync Server API

```http
GET /backup/{user_id}/meta
Response: {
  "last_modified": "2024-01-01T00:00:00Z",
  "file_hash": "sha256:abc123..."
}

GET /backup/{user_id}
Response: (binary SQLite file)

PUT /backup/{user_id}
Body: (binary SQLite file)
Response: { "status": "ok" }
```

## Client-Side Resolution

All conflict resolution happens on the client:

| Scenario | Resolution |
|----------|------------|
| Same row edited on 2 devices | Last-write-wins (compare `changed_at`) |
| Row deleted on one device, edited on other | Delete wins |
| Same row inserted on 2 devices | Keep both (different row IDs) |
