use diesel::prelude::*;

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

#[derive(Serialize, Deserialize, Type, AsChangeset)]
#[diesel(table_name = notes)]
pub struct UpdateNote {
    pub id: i32,
    pub content: String,
    pub updated_at: i64,
}
