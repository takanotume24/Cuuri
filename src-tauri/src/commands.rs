use crate::database::Database;
use crate::get_db_connection::get_db_connection;
use crate::models::{ChatHistory, NewChatHistory};
use crate::schema::chat_history::dsl::*;
use chrono::Utc;
use diesel::prelude::*;

use serde_json::json;
use tauri::State;

#[tauri::command]
pub async fn chat_gpt(
    input_session_id: String,
    message: String,
    model: String,
    api_key: String,
    db: State<'_, Database>,
) -> Result<String, String> {
    // Use the helper function to get a mutable connection
    let mut conn = get_db_connection(&db)?;

    // Pass the mutable reference to Diesel operations
    let session_history = chat_history
        .filter(session_id.eq(&input_session_id))
        .order(created_at.asc())
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    // Build messages vector using functional style
    let mut messages: Vec<_> = session_history
        .iter()
        .flat_map(|entry| {
            vec![
                json!({"role": "user", "content": entry.question.clone()}),
                json!({"role": "assistant", "content": entry.answer.clone()}),
            ]
        })
        .collect();
    messages.push(json!({"role": "user", "content": message.clone()}));

    println!("{:?}", messages);

    // Create request body
    let request_body = json!({
        "model": model,
        "messages": messages,
    });

    // Send request to OpenAI
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    println!("{:?}", json);

    // Extract response content
    let response = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "No response from API".to_string())?;

    // Create new chat history entry
    let now = Utc::now().naive_utc();
    let new_chat = NewChatHistory {
        session_id: &input_session_id,
        question: &message,
        answer: response,
        created_at: now,
    };

    // Insert new entry into the database
    diesel::insert_into(chat_history)
        .values(&new_chat)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(response.to_string())
}
