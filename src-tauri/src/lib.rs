use specta_typescript::Typescript;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};

use crate::commands::notes::{create_note, delete_note, get_notes, update_note};
use crate::commands::users::get_all_users;
use crate::db::setup_data_base;

pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;

struct UserSession {
    user_id: i32
}

pub struct AppState {
    session: Option<UserSession>
}

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
            app.manage(AppState{ session: Some(UserSession{ user_id: 1})});
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
