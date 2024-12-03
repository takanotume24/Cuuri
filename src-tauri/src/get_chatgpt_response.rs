use crate::app_type::ChatResponse;
use crate::database::Database;
use crate::get_db_connection::get_db_connection;
use crate::models::{ChatHistory, NewChatHistory};
use crate::schema::chat_history::dsl::*;
use chrono::Utc;
use diesel::prelude::*;

use serde_json::json;
use tauri::State;
// Define a struct to hold both the response and the timestamp

#[tauri::command]
pub async fn get_chatgpt_response(
    input_session_id: String,
    message: String,
    base64_images: Option<Vec<String>>,
    model: String,
    api_key: String,
    db: State<'_, Database>,
) -> Result<ChatResponse, String> {
    // Change return type here
    let mut conn = get_db_connection(&db)?;

    let session_history = chat_history
        .filter(session_id.eq(&input_session_id))
        .order(created_at.asc())
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    let mut messages: Vec<_> = session_history
        .iter()
        .flat_map(|entry| {
            vec![
                json!({
                    "role": "user",
                    "content": [
                        {"type": "text", "text": entry.question.clone()}
                    ]
                }),
                json!({
                    "role": "assistant",
                    "content": [
                        {"type": "text", "text": entry.answer.clone()}
                    ]
                }),
            ]
        })
        .collect();

    let mut user_content = vec![json!({
        "type": "text",
        "text": message.clone()
    })];

    if let Some(images) = base64_images {
        for image_data in images {
            user_content.push(json!({
                "type": "image_url",
                "image_url": {
                    "url": format!("data:image/jpeg;base64,{}", image_data)
                }
            }));
        }
    }

    messages.push(json!({
        "role": "user",
        "content": user_content
    }));

    println!("{:?}", messages);

    let request_body = json!({
        "model": model,
        "messages": messages,
    });

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

    let response = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "No response from API".to_string())?;

    let now = Utc::now().naive_utc();
    let new_chat = NewChatHistory {
        session_id: &input_session_id,
        question: &message,
        answer: response,
        created_at: now,
    };

    diesel::insert_into(chat_history)
        .values(&new_chat)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    // Return the response along with the timestamp
    Ok(ChatResponse {
        response: response.to_string(),
        created_at: now.to_string(), // Convert timestamp to string
    })
}
