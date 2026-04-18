# Alternatives to Per-User SQLite Backup

## Overview

Other sync approaches considered for MeNote:

## 1. CRDT-Based Sync (automerge / yrs)

**Concept**: Use Conflict-free Replicated Data Types for automatic conflict resolution.

**Pros**:
- Mathematical guarantee of eventual consistency
- No server-side conflict logic
- Works offline perfectly

**Cons**:
- Complex to implement
- Must restructure data model for CRDT
- Larger payload (stores change history)

**For MeNote**: Overkill since per-user backup doesn't need real-time collaboration.

## 2. PostgreSQL Backend (Electric SQL)

**Concept**: Replace SQLite with PostgreSQL, use Electric SQL sync layer.

**Pros**:
- Real sync with partial replication
- Server can query user data
- Mature ecosystem

**Cons**:
- No Rust/Tauri client
- Requires running Electric Service
- Heavy infrastructure

**For MeNote**: Not viable - no Tauri/Rust client exists.

## 3. BaaS (Supabase / Firebase)

**Concept**: Use managed backend-as-a-service for sync.

**Pros**:
- Managed infrastructure
- Real-time sync built-in
- Authentication included

**Cons**:
- External dependency
- Cost at scale
- Data leaves user's device

**For MeNote**: Goes against privacy-first, local-first philosophy.

## 4. Git-Based Sync (Obsidian Sync pattern)

**Concept**: Sync via git repository (GitHub Gist, self-hosted git).

**Pros**:
- Version history built-in
- Free (GitHub Gist is free)
- Familiar technology

**Cons**:
- Git conflicts need resolution
- Not designed for binary/SQLite files
- Git operations can be slow

**For MeNote**: Git Gist could work for simple backups but conflicts are messy.

## 5. WebDAV Sync

**Concept**: Sync to WebDAV server (Nextcloud, etc.).

**Pros**:
- Standard protocol
- Many server options
- Built-in auth

**Cons**:
- Still whole-file sync
- Conflict resolution unclear
- Not designed for SQLite

**For MeNote**: Similar tradeoffs to our approach but with more server requirements.

## Why We Chose Per-User SQLite Backup

| Factor | Decision |
|--------|----------|
| Complexity | Simple server (just file storage) |
| Infrastructure | Any VPS ($5/mo) or self-hosted |
| User data | User owns their SQLite file |
| Privacy | No external service required |
| Implementation | All logic in Rust (fits MeNote stack) |
| Tradeoffs | Acceptable for per-user, non-realtime backup |

## Decision: Per-User SQLite Backup

Our approach wins because:
1. **Server is dumb** - no sync logic, just file storage
2. **Fits our stack** - Rust handles everything
3. **Simple hosting** - any file storage works
4. **User owns data** - SQLite file is theirs
5. **Acceptable tradeoffs** - for notes app, offline/ontime is fine
