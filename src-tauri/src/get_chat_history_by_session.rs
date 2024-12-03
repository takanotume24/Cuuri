use crate::database::Database;
use crate::get_db_connection::get_db_connection;
use crate::models::ChatHistory;
use crate::schema::chat_history::dsl::*;
use diesel::prelude::*;
use tauri::State;
use crate::app_type::RawDatabaseChatEntry;
use tauri::async_runtime::block_on;

#[tauri::command]
pub fn get_chat_history_by_session(
    db: State<'_, Database>,
    target_session_id: String,
) -> Result<Vec<RawDatabaseChatEntry>, String> {
    // Use block_on to run the async code synchronously
    block_on(async {
        // Use the helper function to get a mutable connection
        let mut conn = get_db_connection(&db)?;

        // Filter the chat history by the specific session_id
        let results = chat_history
            .filter(session_id.eq(target_session_id))
            .load::<ChatHistory>(&mut conn)
            .map_err(|e| e.to_string())?;

        // Transform results into a vector of RawDatabaseChatEntry
        let rows: Vec<RawDatabaseChatEntry> = results
            .into_iter()
            .map(|chat| RawDatabaseChatEntry {
                session_id: chat.session_id.clone(),
                question: chat.question.clone(),
                answer: chat.answer.clone(),
                created_at: chat.created_at.to_string(),
            })
            .collect();

        Ok(rows)
    })
}
