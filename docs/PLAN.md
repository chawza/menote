# Frontend
- Sveltekit

# Backend
- Tauri
- Sqlite
- Diesel ORM

# Bridge
- use ts rust to genreate FE typing from BE

# Models
create a simple note taking app that has these models
- Note
  - description (required, max 1000 chars, support md)
    - may contains #tag as free text and should create or update Tag relation save
  - created
  - user
- User, multi user with email and password and disply name
- Tag, refernce to note(s) for grouping and indexing
