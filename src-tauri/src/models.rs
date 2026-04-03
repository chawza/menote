use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub display_name: String,
    pub created_at: i64,
}

#[derive(Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub id: Option<i32>,
    pub user_id: i32,
    pub content: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
