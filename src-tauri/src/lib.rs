use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::dto::UserData;


pub mod models;
pub mod schema;
pub mod dto;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Failed to connect to database")
}

#[specta::specta]
#[tauri::command]
pub fn get_all_users() -> Vec<dto::UserData> {
    use crate::models::User;

    use self::schema::users::dsl::*;
    let conn = &mut establish_connection();
    let fetched = users.select(User::as_select()).load(conn).expect("connection error");
    let results = fetched.into_iter().map(|user| UserData {
        id: user.id.expect("user id expected from database"),
        email: user.email,
        display_name: user.display_name,
        created_at: user.created_at,
    }).collect();
    results
}
