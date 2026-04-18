# Remote Sync — Per-User SQLite Backup

## Architecture Overview

MeNote uses a **per-user SQLite backup** model for remote sync:

- Each user has one `.sqlite` file that lives on their device
- A backup copy lives on a remote server (simple file storage)
- Conflict resolution happens on the **client** (device), not the server
- The server is "dumb" — it just stores files, no sync logic

## Sync Model

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
                            │
              User A's backup (SQLite file)
              No conflict resolution logic here.
              Just stores the file.
```

## How Sync Works

### The Problem Without sync_log

If you just blindly upload/download SQLite files:

```
Device A                          Device B
┌─────────────┐                  ┌─────────────┐
│ Note "Hi"   │                  │ Note "Hi"   │
│ updated at  │                  │ updated at  │
│ 10:00       │                  │ 10:00       │
└─────────────┘                  └─────────────┘

10:01  Edit "Hello"                             
10:02               Edit "World"
10:03  Offline                              Offline
10:04  Come online
       What do I do now?
       - Overwrite with "Hello"? 
       - How do I know B changed it too?
```

Without tracking, you don't know:
- Device B also changed the same note
- You need to resolve conflict, not just overwrite
- Which version is "truth"?

### The Solution: sync_log Table

Every INSERT/UPDATE/DELETE writes to a `sync_log` table:

```sql
CREATE TABLE sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    row_id TEXT NOT NULL,
    operation TEXT NOT NULL,  -- INSERT, UPDATE, DELETE
    changed_at TEXT NOT NULL, -- ISO8601 timestamp
    old_values TEXT,          -- JSON of previous state
    new_values TEXT,          -- JSON of new state
    synced INTEGER DEFAULT 0  -- boolean
);
```

### Sync Process

```
1. Each device maintains its own sync_log
   
2. On sync:
   - Device A sends its unsynced changes (sync_log entries)
   - Device B sends its unsynced changes
   - Each device compares logs and detects conflicts
   
3. Conflict Resolution (client-side):
   - Same row edited on 2 devices → Last-write-wins
   - Row deleted on one, edited on other → Delete wins
   - Same row inserted on 2 devices → Keep both

4. After resolution:
   - Merged SQLite uploaded to server
   - Other devices pull on next sync
```

## Why This Approach?

| Pros | Cons |
|------|------|
| Simple server (just file storage) | Whole file on every sync |
| No server-side conflict logic | No partial sync |
| Client can resolve however it wants | Need app open to sync |
| Works with self-hosted server | |
| User owns their data | |

## What We CAN'T Do

- ❌ Real-time sync (only on-demand)
- ❌ Cross-user collaboration (per-user isolated)
- ❌ Partial sync (whole file always)
- ❌ Automatic background sync (requires app open)
- ❌ Web dashboard (server stores raw SQLite only)
