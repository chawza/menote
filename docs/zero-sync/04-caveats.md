# Caveats of Remote Sync

## Overview

The per-user SQLite backup approach has these caveats:

## 1. Whole File Sync

### Problem: Entire SQLite uploaded/downloaded each sync

Since the server is "dumb" (stores whole files), you can't do partial sync:

```
First sync:    Upload 50MB SQLite
Second sync:   Upload 50MB SQLite (even if only 1KB changed)
```

### Mitigation
- Compress SQLite before upload (SQLite compresses well)
- Track changes at row level but still upload whole file
- Accept that sync will be slow on mobile

## 2. No Real-Time Sync

### Problem: Sync only when app is open

There's no background sync service:

```
User A edits note → App closed → No sync happens
User B edits same note → User A opens app → CONFLICT
```

### Mitigation
- Clear UI about "last synced X minutes ago"
- Make sync button prominent
- Consider auto-sync on app foreground

## 3. Conflict Resolution is Client-Side

### Problem: Each device may resolve differently

If Device A and Device B resolve conflicts differently:

```
Device A: keeps local "Hello" (wrong resolution)
Device B: keeps remote "World" (correct resolution)
After sync: Both should have "World" but A might not upload
```

### Mitigation
- Use deterministic last-write-wins (compare timestamps)
- Always upload after resolving (even if you "lost")
- Test conflict scenarios thoroughly

## 4. Server is Unprotected

### Problem: SQLite file is not encrypted on server

Anyone with access to server storage can read all data:

```bash
# Server storage:
/backup/user123.sqlite  # Plain SQLite file

# Anyone with server access can:
sqlite3 user123.sqlite "SELECT * FROM notes"
```

### Mitigation
- Use HTTPS for transport encryption
- Consider encrypting SQLite file itself before upload
- Self-host server in trusted environment

## 5. No Partial Restore

### Problem: Can't restore single note

You can only restore the entire database:

```
User: "I accidentally deleted note #123 yesterday"
Server: "I only have full backup, no per-note history"
```

### Mitigation
- Keep local backups before sync
- Implement soft deletes (deleted_at timestamp)
- Consider periodic full exports

## 6. Device ID Generation

### Problem: New device needs way to identify itself

First sync on a new device:

```
New Device: "I want to sync user Alice's data"
Server: "Which user? How do I know you're Alice?"
```

### Mitigation
- User provides user_id/token manually
- Simple shared secret approach
- No built-in auth (user sets up server themselves)

## Summary Table

| Caveat | Severity | Workaround |
|--------|----------|------------|
| Whole file sync | Medium | Compress before upload |
| No real-time | High | Auto-sync on foreground |
| Client-side conflict | Medium | Deterministic resolution |
| Unencrypted on server | High | Self-host, HTTPS, encrypt |
| No partial restore | Low | Soft deletes, local backups |
| Manual device auth | Low | Token-based setup |
