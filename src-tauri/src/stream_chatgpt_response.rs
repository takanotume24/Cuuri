use crate::app_type::ChatResponse;
use crate::establish_connection::establish_connection;
use crate::get_database_path::get_database_path;
use crate::models::{ChatHistory, NewChatHistory};
use crate::schema::chat_histories::dsl::*;
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use tauri::{Emitter, Window};

fn fetch_session_history(
    conn: &mut SqliteConnection,
    input_session_id: &String,
) -> Result<Vec<ChatHistory>, String> {
    chat_histories
        .filter(session_id.eq(input_session_id))
        .order(created_at.asc())
        .load::<ChatHistory>(conn)
        .map_err(|e| e.to_string())
}

fn build_messages_from_history(session_history: &[ChatHistory]) -> Vec<serde_json::Value> {
    session_history
        .iter()
        .flat_map(|entry| {
            vec![
                json!({
                    "role": "user",
                    "content": [
                        { "type": "text", "text": entry.question.clone() }
                    ]
                }),
                json!({
                    "role": "assistant",
                    "content": [
                        { "type": "text", "text": entry.answer.clone() }
                    ]
                }),
            ]
        })
        .collect()
}

fn build_user_message(message: &String, base64_images: Option<Vec<String>>) -> serde_json::Value {
    let mut user_content = vec![json!({ "type": "text", "text": message })];
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
    json!({ "role": "user", "content": user_content })
}

fn build_request_body(model: &String, messages: Vec<serde_json::Value>) -> serde_json::Value {
    json!({
        "model": model,
        "messages": messages,
        "stream": true
    })
}

async fn fetch_streaming_response(
    request_body: &serde_json::Value,
    api_key: &String,
) -> Result<reqwest::Response, String> {
    let client = reqwest::Client::new();
    client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))
}

async fn process_stream_response(
    mut res: reqwest::Response,
    window: &Window,
) -> Result<String, String> {
    let mut full_response = String::new();
    while let Some(chunk) = res.chunk().await.map_err(|e| e.to_string())? {
        let chunk_str = match std::str::from_utf8(&chunk) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for line in chunk_str.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(json_str) = line.strip_prefix("data: ") {
                if json_str == "[DONE]" {
                    break;
                }
                let parsed: serde_json::Value = match serde_json::from_str(json_str) {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                if let Some(content) = parsed["choices"][0]["delta"]["content"].as_str() {
                    full_response.push_str(content);
                    let _ = window.emit("token", content.to_string());
                }
            }
        }
    }
    Ok(full_response)
}

fn store_response_to_db(
    conn: &mut SqliteConnection,
    input_session_id: &String,
    question_text: &String,
    full_response: &String,
) -> Result<(), String> {
    let now = Utc::now().naive_utc();
    let new_chat = NewChatHistory {
        session_id: input_session_id,
        question: question_text,
        answer: full_response,
        created_at: now,
    };
    diesel::insert_into(chat_histories)
        .values(&new_chat)
        .execute(conn)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn stream_chatgpt_response(
    window: Window,
    input_session_id: String,
    message: String,
    base64_images: Option<Vec<String>>,
    model: String,
    api_key: String,
) -> Result<ChatResponse, String> {
    let database_path = get_database_path().map_err(|e| e.to_string())?;
    let mut conn = establish_connection(&database_path).map_err(|e| e.to_string())?;

    // Step 1: fetch session history
    let session_history = fetch_session_history(&mut conn, &input_session_id)?;

    // Step 2: build messages from history
    let mut messages = build_messages_from_history(&session_history);

    // Step 3: add user message
    let user_msg = build_user_message(&message, base64_images);
    messages.push(user_msg);

    // Step 4: build request body
    let request_body = build_request_body(&model, messages);

    // Step 5: send streaming request
    let res = fetch_streaming_response(&request_body, &api_key).await?;

    // Step 6: process response
    let full_response = process_stream_response(res, &window).await?;

    // Step 7: store to DB
    store_response_to_db(&mut conn, &input_session_id, &message, &full_response)?;

    // Step 8: return
    let now = Utc::now().naive_utc();
    Ok(ChatResponse {
        response: full_response,
        created_at: now.to_string(),
    })
}
