# Preparation for Remote Sync

## Overview

Adding remote sync to MeNote requires:
1. Adding `sync_log` table to track changes
2. Adding `sync_state` table to store sync metadata
3. Wrapping all data mutations to automatically log changes
4. Adding Tauri commands for sync operations
5. Adding HTTP client for server communication

## Step 1: Create Sync Migrations

Create a new Diesel migration for the sync tables:

```bash
cd src-tauri
diesel migration generate add_sync_tables
```

Edit `migrations/2024-01-01-000000_add_sync_tables/up.sql`:

```sql
-- Sync log: tracks all changes for sync
CREATE TABLE sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    row_id TEXT NOT NULL,
    operation TEXT NOT NULL CHECK(operation IN ('INSERT', 'UPDATE', 'DELETE')),
    changed_at TEXT NOT NULL,
    old_values TEXT,
    new_values TEXT,
    synced INTEGER DEFAULT 0 NOT NULL
);

CREATE INDEX idx_sync_log_synced ON sync_log(synced);
CREATE INDEX idx_sync_log_table_row ON sync_log(table_name, row_id);

-- Sync state: key-value store for sync metadata
CREATE TABLE sync_state (
    key TEXT PRIMARY KEY,
    value TEXT
);

-- Initialize with device info
INSERT INTO sync_state (key, value) VALUES ('device_id', '<uuid-v4>');
INSERT INTO sync_state (key, value) VALUES ('device_name', '<hostname>');
INSERT INTO sync_state (key, value) VALUES ('last_sync_time', '');
INSERT INTO sync_state (key, value) VALUES ('server_url', '');
```

Edit `migrations/2024-01-01-000000_add_sync_tables/down.sql`:

```sql
DROP TABLE IF EXISTS sync_log;
DROP TABLE IF EXISTS sync_state;
```

## Step 2: Add Rust Dependencies

```toml
# src-tauri/Cargo.toml
[dependencies]
# Add these:
reqwest = { version = "0.12", features = ["json", "blocking"] }
tokio = { version = "1", features = ["full"] }
sha2 = "0.10"
hex = "0.4"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
```

## Step 3: Create Sync Module

Create `src-tauri/src/sync.rs`:

```rust
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncLogEntry {
    pub id: i64,
    pub table_name: String,
    pub row_id: String,
    pub operation: String,
    pub changed_at: String,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub device_id: String,
    pub device_name: String,
    pub last_sync_time: Option<String>,
    pub server_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub table: String,
    pub row_id: String,
    pub operation: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub timestamp: String,
}

pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
```

## Step 4: Wrap Data Mutations with log_change()

Every INSERT, UPDATE, DELETE in your Rust code needs to call `log_change()`:

```rust
// Example: update_note command
pub fn update_note(
    conn: &mut SqliteConnection,
    note: Note,
    user_id: i32,
) -> Result<Note, AppError> {
    // 1. Get old values for sync_log
    let old_note = notes::table.find(note.id).first::<Note>(conn)?;
    
    // 2. Do the actual update
    diesel::update(notes::table.find(note.id))
        .set((
            notes::title.eq(&note.title),
            notes::content.eq(&note.content),
            notes::updated_at.eq(chrono::Utc::now()),
        ))
        .execute(conn)?;
    
    // 3. Log the change
    log_change(
        conn,
        "notes",
        &note.id.to_string(),
        "UPDATE",
        Some(serde_json::to_value(&old_note).ok()),
        Some(serde_json::to_value(&note).ok()),
    )?;
    
    Ok(note)
}

fn log_change(
    conn: &mut SqliteConnection,
    table_name: &str,
    row_id: &str,
    operation: &str,
    old_values: Option<serde_json::Value>,
    new_values: Option<serde_json::Value>,
) -> Result<(), AppError> {
    diesel::insert_into(sync_log::table)
        .values((
            sync_log::table_name.eq(table_name),
            sync_log::row_id.eq(row_id),
            sync_log::operation.eq(operation),
            sync_log::changed_at.eq(chrono::Utc::now().to_rfc3339()),
            sync_log::old_values.eq(old_values.map(|v| v.to_string())),
            sync_log::new_values.eq(new_values.map(|v| v.to_string())),
            sync_log::synced.eq(false),
        ))
        .execute(conn)?;
    Ok(())
}
```

## Step 5: Create Sync Tauri Commands

```rust
// src-tauri/src/lib.rs

#[derive(Serialize)]
struct SyncResult {
    status: String,
    changes_applied: usize,
    conflicts_resolved: usize,
}

#[tauri::command]
async fn sync_now(user_id: i32) -> Result<SyncResult, String> {
    // 1. Get unsynced changes
    // 2. Download server SQLite
    // 3. Compare and merge
    // 4. Upload merged result
    // 5. Mark as synced
}

#[tauri::command]
fn get_sync_status() -> Result<SyncState, String> {
    // Return current sync state
}

#[tauri::command]
fn set_sync_server(url: String) -> Result<(), String> {
    // Store server URL in sync_state
}
```

## Step 6: Frontend Sync UI

```typescript
// src/lib/components/SyncStatus.svelte
<script>
  import { commands } from '$lib/commands';
  
  let syncStatus = $state({ lastSync: null, pending: 0 });
  
  async function triggerSync() {
    const result = await commands.sync_now();
    if (result.status === 'ok') {
      toast.success(`Synced: ${result.changes_applied} changes`);
    }
  }
</script>

<button onclick={triggerSync}>
  {#if syncStatus.pending > 0}
    <Icon name="cloud-upload" />
    {syncStatus.pending} pending
  {:else}
    <Icon name="cloud-done" />
    Synced
  {/if}
</button>
```

## Verification Checklist

- [ ] `sync_log` table created with indexes
- [ ] `sync_state` table created
- [ ] All mutations call `log_change()`
- [ ] `sync_now()` command implemented
- [ ] `get_sync_status()` command implemented
- [ ] `set_sync_server()` command implemented
- [ ] HTTP client added for server communication
- [ ] Sync UI component created
- [ ] First-time sync flow (upload vs download) handled
