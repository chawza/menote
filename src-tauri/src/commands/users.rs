use crate::{
    db::establish_connection,
    error::AppError,
    models::users::{User, UserData},
};
use diesel::prelude::*;

#[specta::specta]
#[tauri::command]
pub fn get_all_users() -> Result<Vec<UserData>, AppError> {
    use crate::schema::users::dsl::*;

    let conn = &mut establish_connection();
    let fetched = users.select(User::as_select()).load(conn)?;
    let results: Vec<UserData> = fetched
        .into_iter()
        .map(|user| {
            Ok(UserData {
                id: user.id.ok_or_else(|| AppError::missing_id("User"))?,
                email: user.email,
                display_name: user.display_name,
                created_at: user.created_at,
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;
    Ok(results)
}
