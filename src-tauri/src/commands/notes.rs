use crate::{
    AppState, db::establish_connection, error::AppError, models::notes::{NewNote, Note, NoteDetail, UpdateNote}
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

#[specta::specta]
#[tauri::command]
pub fn create_note(note: NewNote, state: tauri::State<AppState>) -> Result<NoteDetail, AppError> {
    use crate::schema::notes::dsl::*;
    use diesel::insert_into;

    let session = state.session.as_ref().ok_or_else(|| AppError::unauthorized("Need Log in"))?;

    if note.user_id != session.user_id {
        return Err(AppError::unauthorized("Cannot create note for another user"));
    }

    let conn = &mut establish_connection();
    let created_note: Note = insert_into(notes).values(&note).get_result(conn)?;

    Ok(NoteDetail {
        id: created_note
            .id
            .ok_or_else(|| AppError::missing_id("Note"))?,
        user_id: created_note.user_id,
        content: created_note.content,
        created_at: created_note.created_at,
        updated_at: created_note.updated_at,
    })
}

pub fn get_note_by_id(note_id: i32, conn: &mut SqliteConnection) -> Result<NoteDetail, AppError> {
    use crate::schema::notes;
    let note = notes::table
        .filter(notes::id.eq(note_id))
        .first::<Note>(conn)?;
    Ok(NoteDetail {
        id: note.id.ok_or_else(|| AppError::missing_id("Note"))?,
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
pub fn update_note(update_note: UpdateNote, state: tauri::State<AppState>) -> Result<NoteDetail, AppError> {
    use crate::schema::notes::dsl::*;
    state.session.as_ref().ok_or_else(|| AppError::unauthorized("Need Log in"))?;

    let conn = &mut establish_connection();
    let note = match get_note_by_id(update_note.id, conn) {
        Ok(note) => note,
        Err(e) => return Err(e),
    };
    if note.user_id != state.session.as_ref().unwrap().user_id {
        return Err(AppError::unauthorized("Cannot update note for another user"));
    }

    diesel::update(notes.filter(id.eq(note.id)))
        .set(&update_note)
        .execute(conn)?;
    get_note_by_id(note.id, conn)
}

#[specta::specta]
#[tauri::command]
pub fn delete_note(note_id: i32) -> Result<bool, AppError> {
    let conn = &mut establish_connection();
    let affected = delete_by_id(note_id, conn)?;
    if affected == 0 {
        return Err(AppError::note_not_found(note_id));
    }
    Ok(true)
}

#[specta::specta]
#[tauri::command]
pub fn get_notes(state: tauri::State<AppState>) -> Result<Vec<NoteDetail>, AppError> {
    use crate::schema::notes;
    let session = state.session.as_ref().ok_or_else(|| AppError::unauthorized("Need Log in"))?;

    let conn = &mut establish_connection();

    let fetched_notes: Vec<Note> = notes::table
        .filter(notes::user_id.eq(session.user_id))
        .order_by(notes::created_at.desc())
        .load::<Note>(conn)?;

    let results = fetched_notes
        .iter()
        .map(|note| {
            Ok(NoteDetail {
                id: note.id.ok_or_else(|| AppError::missing_id("Note"))?,
                user_id: note.user_id,
                content: note.content.clone(),
                created_at: note.created_at,
                updated_at: note.updated_at,
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    Ok(results)
}
