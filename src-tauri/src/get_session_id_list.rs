use crate::app_type::SessionId;
use crate::establish_connection::establish_connection;
use crate::get_database_path::get_database_path;
use crate::schema::chat_histories::dsl::*;
use diesel::prelude::*;
use tauri::async_runtime::block_on;

#[tauri::command]
pub fn get_session_id_list() -> Result<Vec<SessionId>, String> {
    block_on(async {
        let database_path = get_database_path().map_err(|e| e.to_string())?;
        let mut conn = establish_connection(&database_path);

        let results = chat_histories
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
