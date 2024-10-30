use reqwest::Client;
use serde_json::json;
use std::env;
use dotenvy::dotenv;
use tauri::State;
use tokio::sync::Mutex; // tokio::sync::Mutex を使用

struct ApiKey(String);

// 会話履歴を保持するための構造体
struct ChatHistory(Mutex<Vec<serde_json::Value>>);

#[tauri::command]
async fn chat_gpt(message: String, state: State<'_, ApiKey>, history: State<'_, ChatHistory>) -> Result<String, String> {
    let client = Client::new();

    // 新しいメッセージを履歴に追加
    let mut history_lock = history.0.lock().await;
    history_lock.push(json!({"role": "user", "content": message}));

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": history_lock.clone(), // クローンを作成
    });

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.0))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let response = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No response from API".to_string())?;

    // AI の応答を履歴に追加
    history_lock.push(json!({"role": "assistant", "content": response}));

    Ok(response.to_string())
}

pub fn run() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .manage(ChatHistory(Mutex::new(Vec::new())))
        .invoke_handler(tauri::generate_handler![chat_gpt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
