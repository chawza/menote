use diesel::prelude::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub display_name: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Type)]
pub struct UserData {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub created_at: i64,
}
