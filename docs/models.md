# Data Models

This document describes the data models used in the MeNote application.

## Overview

MeNote uses three main data models:
- **User**: Multi-user support with authentication
- **Note**: Markdown notes with tag support
- **Tag**: Labels extracted from note content for organization

## Entity Relationship Diagram

```
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│      User       │       │      Note       │       │       Tag       │
├─────────────────┤       ├─────────────────┤       ├─────────────────┤
│ PK id           │──┐    │ PK id           │    ┌──│ PK id           │
│    email        │  │    │ FK user_id      │────┘   │    name         │
│    password_hash│  └───>│    content      │    ┌──┘    color        │
│    display_name │       │    created_at   │    │    ├─────────────────┤
│    created_at   │       │    updated_at   │    │    │  note_tags      │
└─────────────────┘       └─────────────────┘    │    ├─────────────────┤
                                                 └───>│ FK note_id      │
                                                      │ FK tag_id       │
                                                      └─────────────────┘
```

## Models

### User

Represents an application user with authentication credentials.

#### Attributes

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | Integer | Primary Key, Auto-increment | Unique identifier |
| `email` | String | Unique, Not Null, Max 255 chars | User's email address |
| `password_hash` | String | Not Null | Bcrypt hashed password |
| `display_name` | String | Max 100 chars | User's display name |
| `created_at` | DateTime | Not Null, Default: now | Account creation timestamp |

#### Diesel Usage

Uses Diesel derive macros for database operations:
- `Queryable` - For reading from database
- `Insertable` - For creating new records
- `TS` (from ts-rs) - Auto-generates TypeScript types

Password hash is marked with `#[ts(skip)]` to prevent exposing it to the frontend.

### Note

Represents a markdown note created by a user.

#### Attributes

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | Integer | Primary Key, Auto-increment | Unique identifier |
| `user_id` | Integer | Foreign Key → users.id, Not Null | Note owner |
| `content` | Text | Not Null, Max 1000 chars | Markdown content with #tags |
| `created_at` | DateTime | Not Null, Default: now | Creation timestamp |
| `updated_at` | DateTime | Not Null, Default: now | Last update timestamp |

#### Diesel Usage

Uses Diesel derive macros:
- `Queryable` - For reading records
- `Identifiable` - For primary key handling
- `Associations` - For relationships (belongs_to User)
- `Insertable` - For creating records
- `TS` - For TypeScript type generation

#### Tag Extraction

Notes automatically extract tags from content using the pattern `#tagname`:

- Tags are parsed from note content using regex: `#([a-zA-Z0-9_-]+)`
- Extracted tags are created if they don't exist
- Tag relationships are updated when note is saved
- Tags persist even if removed from content (for history)

### Tag

Represents a label used for organizing and indexing notes.

#### Attributes

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | Integer | Primary Key, Auto-increment | Unique identifier |
| `name` | String | Unique, Not Null, Max 50 chars | Tag name (lowercase) |
| `color` | String | Nullable, 7 chars (#RRGGBB) | Optional color code |
| `created_at` | DateTime | Not Null, Default: now | Creation timestamp |

#### Diesel Usage

Uses Diesel derive macros:
- `Queryable` - For reading records
- `Identifiable` - For primary key handling
- `Insertable` - For creating records
- `TS` - For TypeScript type generation

### NoteTag (Join Table)

Many-to-many relationship between notes and tags.

#### Attributes

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `note_id` | Integer | Foreign Key → notes.id, Not Null | Reference to note |
| `tag_id` | Integer | Foreign Key → tags.id, Not Null | Reference to tag |
| `created_at` | DateTime | Not Null, Default: now | Relationship timestamp |

#### Diesel Usage

Uses Diesel derive macros for many-to-many join table:
- `Queryable` - For reading records
- `Insertable` - For creating associations
- `Associations` - For both Note and Tag relationships
- `primary_key` - Composite key (note_id, tag_id)

## Database Schema (SQL)

```sql
-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notes table
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL CHECK(length(content) <= 1000),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Tags table
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL UNIQUE,
    color VARCHAR(7),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Note-Tag relationship table
CREATE TABLE note_tags (
    note_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (note_id, tag_id),
    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_notes_user_id ON notes(user_id);
CREATE INDEX idx_notes_created_at ON notes(created_at);
CREATE INDEX idx_note_tags_note_id ON note_tags(note_id);
CREATE INDEX idx_note_tags_tag_id ON note_tags(tag_id);
```

## Business Rules

### User

1. Email must be unique across all users
2. Password must be hashed before storage (never store plaintext)
3. Display name is optional but recommended
4. Users can only access their own notes

### Note

1. Maximum content length: 1000 characters
2. Content supports Markdown formatting
3. Tags are extracted from content using `#tagname` syntax
4. Multiple tags can be added to a single note
5. Tags are case-insensitive (stored in lowercase)
6. When a note is updated, tag relationships are re-evaluated
7. Notes are soft-deleted by user (hard delete not implemented)

### Tag

1. Tag names are unique and stored in lowercase
2. Tags can have an optional color (hex format: #RRGGBB)
3. Tags are automatically created when first used in a note
4. Tags persist even if all associated notes are deleted
5. Tags can be manually created without notes

## Validation Rules

### Note Content

```rust
fn validate_note_content(content: &str) -> Result<(), ValidationError> {
    if content.is_empty() {
        return Err(ValidationError::EmptyContent);
    }
    if content.len() > 1000 {
        return Err(ValidationError::ContentTooLong);
    }
    Ok(())
}
```

### Tag Extraction

```rust
fn extract_tags(content: &str) -> Vec<String> {
    let tag_regex = Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap();
    tag_regex
        .captures_iter(content)
        .map(|cap| cap[1].to_lowercase())
        .collect::<HashSet<_>>()  // Remove duplicates
        .into_iter()
        .collect()
}
```

## Example Usage

### Creating a Note with Tags

```rust
// User creates a note
let content = "Meeting notes #work #project-alpha Discussed timeline for Q4";

// Backend processing
let note = create_note(user_id, content)?;
// Extracted tags: ["work", "project-alpha"]
// Tags are created and linked to the note automatically
```

### Querying Notes by Tag

```rust
// Get all notes with a specific tag
let notes = notes::table
    .inner_join(note_tags::table)
    .inner_join(tags::table)
    .filter(tags::name.eq("work"))
    .filter(notes::user_id.eq(current_user_id))
    .load::<(Note, Tag)>(conn)?;
```

## Migration Strategy

When adding new models or modifying existing ones:

1. Create a new Diesel migration
2. Update the Rust model structs
3. Add TS derive macros for new types
4. Generate TypeScript bindings
5. Update frontend types and components

## References

- [Diesel Documentation](https://diesel.rs/guides/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [ts-rs Documentation](https://github.com/Aleph-Alpha/ts-rs)
