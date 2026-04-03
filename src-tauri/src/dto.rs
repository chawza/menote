use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::schema::notes;

#[derive(Serialize, Deserialize, Type)]
pub struct UserData {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Type, Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote {
    pub user_id: i32,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Type)]
pub struct NoteDetail {
    pub id: i32,
    pub user_id: i32,
    pub content: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
