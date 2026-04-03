use std::env;

use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenv::dotenv;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

use crate::{
    dto::{NewNote, NoteDetail, UserData},
    models::Note,
};

pub mod dto;
pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        get_all_users,
        get_notes,
        create_note
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
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
}

#[specta::specta]
#[tauri::command]
fn get_all_users() -> Vec<dto::UserData> {
    use crate::models::User;

    use self::schema::users::dsl::*;
    let conn = &mut establish_connection();
    let fetched = users
        .select(User::as_select())
        .load(conn)
        .expect("connection error");
    let results = fetched
        .into_iter()
        .map(|user| UserData {
            id: user.id.expect("user id expected from database"),
            email: user.email,
            display_name: user.display_name,
            created_at: user.created_at,
        })
        .collect();
    results
}

#[specta::specta]
#[tauri::command]
fn create_note(note: NewNote) -> dto::NoteDetail {
    use crate::schema::notes::dsl::*;
    use diesel::insert_into;

    let conn = &mut establish_connection();
    let created_note: Note = insert_into(notes).values(&note).get_result(conn).unwrap();

    NoteDetail {
        id: created_note.id.unwrap(),
        user_id: created_note.user_id,
        content: created_note.content,
        created_at: created_note.created_at,
        updated_at: created_note.updated_at,
    }
}

#[specta::specta]
#[tauri::command]
fn get_notes(user_id: i32) -> Vec<dto::NoteDetail> {
    use crate::schema::notes;

    let conn = &mut establish_connection();

    let fetched_notes: Vec<Note> = notes::table
        .filter(notes::id.eq(user_id))
        .order_by(notes::created_at.desc())
        .load::<Note>(conn)
        .unwrap();

    fetched_notes
        .iter()
        .map(|note| NoteDetail {
            id: note.id.unwrap(),
            user_id: note.user_id,
            content: note.content.clone(),
            created_at: note.created_at,
            updated_at: note.updated_at,
        })
        .collect()
}
