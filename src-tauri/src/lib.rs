use std::env;

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

use crate::{
    error::AppError, models::{notes::Note, users::User},
};

use crate::models::notes;
use crate::models::users;

pub mod error;
pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        get_all_users,
        get_notes,
        create_note,
        update_note,
        delete_note,
    ]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(|app| {
            setup_data_base(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Failed to connect to database")
}

fn setup_data_base(app: &tauri::App) {
    use tauri::Manager;
    let app_dir = app.path().app_data_dir().unwrap();
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");

    let db_path = app_dir.join("menote.sqlite");
    println!("db path: {}", db_path.display());

    env::set_var("DATABASE_URL", &db_path);

    let mut conn = establish_connection();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

#[specta::specta]
#[tauri::command]
fn get_all_users() -> Result<Vec<users::UserData>, AppError> {

    let conn = &mut establish_connection();
    let fetched = users.select(User::as_select()).load(conn)?;
    let results: Vec<users::UserData> = fetched
        .into_iter()
        .map(|user| {
            Ok(UserData {
                id: user
                    .id
                    .ok_or_else(|| AppError::Internal("User missing id".into()))?,
                email: user.email,
                display_name: user.display_name,
                created_at: user.created_at,
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;
    Ok(results)
}

#[specta::specta]
#[tauri::command]
fn create_note(note: notes::NewNote) -> Result<notes::NoteDetail, AppError> {
    use crate::schema::notes::dsl::*;
    use diesel::insert_into;

    let conn = &mut establish_connection();
    let created_note: Note = insert_into(notes).values(&note).get_result(conn)?;

    Ok(NoteDetail {
        id: created_note
            .id
            .ok_or_else(|| AppError::Internal("Created note missing id".into()))?,
        user_id: created_note.user_id,
        content: created_note.content,
        created_at: created_note.created_at,
        updated_at: created_note.updated_at,
    })
}

fn get_note_by_id(note_id: i32, conn: &mut SqliteConnection) -> Result<notes::NoteDetail, AppError> {
    use crate::schema::notes;
    let note = notes::table
        .filter(notes::id.eq(note_id))
        .first::<notes::Note>(conn)?;
    Ok(notes::NoteDetail {
        id: note
            .id
            .ok_or_else(|| AppError::Internal("Note missing id".into()))?,
        user_id: note.user_id,
        content: note.content,
        created_at: note.created_at,
        updated_at: note.updated_at,
    })
}

fn delete_by_id(note_id: i32, conn: &mut SqliteConnection) -> Result<usize, AppError> {
    use crate::schema::notes::dsl::*;
    Ok(diesel::delete(notes.filter(id.eq(note_id))).execute(conn)?)
}

#[specta::specta]
#[tauri::command]
fn update_note(note: notes::UpdateNote) -> Result<notes::NoteDetail, AppError> {
    use crate::schema::notes::dsl::*;
    let conn = &mut establish_connection();
    diesel::update(notes).set(&note).execute(conn)?;
    get_note_by_id(note.id, conn)
}

#[specta::specta]
#[tauri::command]
fn delete_note(note_id: i32) -> Result<bool, AppError> {
    let conn = &mut establish_connection();
    let affected = delete_by_id(note_id, conn)?;
    if affected == 0 {
        return Err(AppError::NotFound("Note not found".into()));
    }
    Ok(true)
}

#[specta::specta]
#[tauri::command]
fn get_notes(user_id: i32) -> Result<Vec<notes::NoteDetail>, AppError> {
    use crate::schema::notes;

    let conn = &mut establish_connection();

    let fetched_notes: Vec<notes::Note> = notes::table
        .filter(notes::user_id.eq(user_id))
        .order_by(notes::created_at.desc())
        .load::<notes::Note>(conn)?;

    let results = fetched_notes
        .iter()
        .map(|note| {
            Ok(notes::NoteDetail {
                id: note
                    .id
                    .ok_or_else(|| AppError::Internal("Note missing id".into()))?,
                user_id: note.user_id,
                content: note.content.clone(),
                created_at: note.created_at,
                updated_at: note.updated_at,
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    Ok(results)
}
