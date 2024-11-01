use crate::config::Config;
use crate::database::{ApiKey, Database};
use crate::models::{ChatHistory, NewChatHistory};
use crate::schema::chat_history::dsl::*;
use chrono::Utc;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn chat_gpt(
    input_session_id: String,
    message: String,
    model: String,
    state: State<'_, ApiKey>,
    db: State<'_, Database>,
) -> Result<String, String> {
    let mut conn = db.pool.get().map_err(|e| e.to_string())?;

    let session_history = chat_history
        .filter(session_id.eq(input_session_id.clone()))
        .order(created_at.asc())
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    let mut messages = Vec::new();
    for entry in session_history {
        messages.push(json!({"role": "user", "content": entry.question}));
        messages.push(json!({"role": "assistant", "content": entry.answer}));
    }
    messages.push(json!({"role": "user", "content": message}));

    println!("{:?}", messages);

    let client = reqwest::Client::new();
    let request_body = json!({
        "model": model,
        "messages": messages,
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.0))
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
        session_id: input_session_id.as_str(),
        question: message.as_str(),
        answer: response,
        created_at: now,
    };

    diesel::insert_into(chat_history)
        .values(&new_chat)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(response.to_string())
}

#[tauri::command]
pub async fn get_chat_history(db: State<'_, Database>) -> Result<Vec<HashMap<String, String>>, String> {
    let mut conn = db.pool.get().map_err(|e| e.to_string())?;

    let results = chat_history
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    let rows: Vec<HashMap<String, String>> = results
        .into_iter()
        .map(|chat| {
            let mut map = HashMap::new();
            map.insert("session_id".to_string(), chat.session_id);
            map.insert("question".to_string(), chat.question);
            map.insert("answer".to_string(), chat.answer);
            map.insert("created_at".to_string(), chat.created_at.to_string());
            map
        })
        .collect();

    Ok(rows)
}

#[tauri::command]
pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

#[tauri::command]
pub async fn get_available_models(state: State<'_, ApiKey>) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", state.0))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to fetch models: HTTP {}", res.status()));
    }

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let mut models: Vec<String> = json["data"]
        .as_array()
        .ok_or("Invalid response format")?
        .iter()
        .filter_map(|model| model["id"].as_str().map(|s| s.to_string()))
        .collect();

    models.sort();

    Ok(models)
}

#[tauri::command]
pub fn get_default_model() -> Result<String, String> {
    dotenv().ok();
    match env::var("DEFAULT_MODEL") {
        Ok(default_model) => Ok(default_model),
        Err(e) => Err(format!("Failed to get DEFAULT_MODEL: {}", e)),
    }
}

#[tauri::command]
pub async fn set_openai_api_key(api_key: String) -> Result<(), String> {
    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".chauri/config.toml");

    let mut config = Config::from_file(config_path.to_str().unwrap())
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    config.openai_api_key = api_key;

    let content =
        toml::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, content)
        .map_err(|e| format!("Failed to write to config file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_openai_api_key() -> Result<String, String> {
    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".chauri/config.toml");

    if !config_path.exists() {
        return Err("Configuration file does not exist".to_string());
    }

    let config = Config::from_file(config_path.to_str().unwrap())
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    Ok(config.openai_api_key)
}
