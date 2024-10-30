use reqwest::Client;
use serde_json::json;
use std::env;
use dotenvy::dotenv;

struct ApiKey(String);

#[tauri::command]
async fn chat_gpt(message: String, state: tauri::State<'_, ApiKey>) -> Result<String, String> {
    let client = Client::new();

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": message}]
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

    Ok(response.to_string())
}

pub fn run() {
    // dotenv を初期化し、.env ファイルを読み込みます
    dotenv().ok();

    // 環境変数から API キーを取得します
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .invoke_handler(tauri::generate_handler![chat_gpt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
