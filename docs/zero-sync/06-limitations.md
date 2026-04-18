# Things We CANNOT Do with This Sync Approach

## Overview

Per-user SQLite backup sync has intentional limitations:

## 1. ❌ Real-Time Sync

```
Device A                          Device B
    │                                  │
    ├─── edit note ──────────────────►│
    │                                  │
    │         ✗ NO REAL-TIME          │
    │         Only on-demand          │
```

**What this means**: Changes only sync when user manually triggers or app opens.

## 2. ❌ Cross-User Collaboration

```
User A's Device                    User B's Device
┌─────────────┐                  ┌─────────────┐
│ User A's   │        ✗          │ User B's   │
│ SQLite     │  ────────────────→│ SQLite     │
└─────────────┘                  └─────────────┘
     │                                  │
     └─── sync to ────► server ◄─── sync ─┘
                      │
                      └─── user_a.sqlite ─┐
                      └─── user_b.sqlite ─┘
                      (completely separate)
```

**What this means**: Each user has their own SQLite file. No sharing between users.

## 3. ❌ Partial Sync

```
┌─────────────────────────────────────────┐
│  Sync always transfers WHOLE SQLite     │
│                                         │
│  Changed: 1 row                          │
│  Uploaded: 50MB (entire file)           │
└─────────────────────────────────────────┘
```

**What this means**: Even if you changed one note, the entire SQLite file is uploaded/downloaded.

## 4. ❌ Automatic Background Sync

```
User closes app                          App not running
    │                                          │
    └─── changes made ──→ ✗ nobody to sync ─────┘
```

**What this means**: App must be open to sync. No background worker.

## 5. ❌ Web Dashboard

```
┌─────────────────┐
│  MeNote Desktop  │  ◄── Only way to access notes
│      App        │
└─────────────────┘
        ✗
        ▼
┌─────────────────┐
│  Browser        │  ◄── Cannot access notes
│  (web UI)       │
└─────────────────┘
        ✗
        ▼
┌─────────────────┐
│  Server         │  ◄── Only stores raw SQLite
│  (backup)       │
└─────────────────┘
```

**What this means**: Server stores raw SQLite, no web interface to browse/edit notes.

## 6. ❌ Selective Sync (Sync Specific Notes)

```
┌─────────────────────────────────────┐
│  All notes always synced together   │
│                                      │
│  - Note #1 (personal)                │
│  - Note #2 (work)      ── sync ──►  │
│  - Note #3 (secret)                  │
│                                      │
│  Cannot sync only #2                 │
└─────────────────────────────────────┘
```

**What this means**: All or nothing. Can't sync only some notes.

## 7. ❌ Server-Side Search/AI

```
┌─────────────────┐         ┌─────────────────┐
│  Local Notes    │         │  Server         │
│                 │   ✗     │  (just storage) │
│  [search here]  │ ──────► │  [cannot search]│
└─────────────────┘         └─────────────────┘
```

**What this means**: All processing is local. Can't do server-side AI or full-text search across all user data.

## Summary

| Cannot Do | Why | Workaround |
|-----------|-----|------------|
| Real-time sync | No persistent connection | Manual sync button |
| Cross-user collaboration | Per-user isolation | Not planned |
| Partial sync | Server is dumb file storage | Accept whole-file |
| Background sync | No sync service | Auto-sync on app open |
| Web dashboard | Server is just backup | N/A |
| Selective sync | Whole file approach | N/A |
| Server-side AI | Data stays local | Use local AI |
