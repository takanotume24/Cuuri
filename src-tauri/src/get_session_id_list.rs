use crate::database::Database;
use crate::get_db_connection::get_db_connection;
use crate::schema::chat_history::dsl::*;
use diesel::prelude::*;
use tauri::State;
use tauri::async_runtime::block_on;
use crate::app_type::SessionId;

#[tauri::command]
pub fn get_session_id_list(
    db: State<'_, Database>,
) -> Result<Vec<SessionId>, String> {
    block_on(async {
        let mut conn = get_db_connection(&db)?;

        let results = chat_history
            .select(session_id)
            .distinct()
            .load::<String>(&mut conn)
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(SessionId)
            .collect();

        Ok(results)
    })
}

