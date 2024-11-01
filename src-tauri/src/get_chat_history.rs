use crate::database::Database;
use crate::get_db_connection::get_db_connection;
use crate::models::ChatHistory;
use crate::schema::chat_history::dsl::*;
use diesel::prelude::*;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn get_chat_history(
    db: State<'_, Database>,
) -> Result<Vec<HashMap<String, String>>, String> {
    // Use the helper function to get a mutable connection
    let mut conn = get_db_connection(&db)?;

    // Pass the mutable reference to Diesel operations
    let results = chat_history
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    // Transform results into a vector of hash maps using functional style
    let rows: Vec<_> = results
        .into_iter()
        .map(|chat| {
            let mut map = HashMap::new();
            map.insert("session_id".to_string(), chat.session_id.clone());
            map.insert("question".to_string(), chat.question.clone());
            map.insert("answer".to_string(), chat.answer.clone());
            map.insert("created_at".to_string(), chat.created_at.to_string());
            map
        })
        .collect();

    Ok(rows)
}
