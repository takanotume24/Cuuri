use crate::app_type::ChatResponse;
use crate::establish_connection::establish_connection;
use crate::get_database_path::get_database_path;
use crate::models::{ChatHistory, NewChatHistory};
use crate::schema::chat_histories::dsl::*;
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use tauri::{Emitter, Window}; // Emitter をインポート

#[tauri::command]
pub async fn stream_chatgpt_response(
    window: Window,
    input_session_id: String,
    message: String,
    base64_images: Option<Vec<String>>,
    model: String,
    api_key: String,
) -> Result<ChatResponse, String> {
    // 1) DB から過去のチャット履歴を取得
    let database_path = get_database_path().map_err(|e| e.to_string())?;
    let mut conn = establish_connection(&database_path).map_err(|e| e.to_string())?;

    let session_history = chat_histories
        .filter(session_id.eq(&input_session_id))
        .order(created_at.asc())
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    // 2) これまでの会話履歴を API 用に組み立て
    let mut messages: Vec<serde_json::Value> = session_history
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

    // 画像を含める場合の処理
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

    // 今回のユーザ入力を追加
    messages.push(json!({
        "role": "user",
        "content": user_content
    }));

    // 3) API リクエストボディ: stream: true を付ける
    let request_body = json!({
        "model": model,
        "messages": messages,
        "stream": true
    });

    // 4) OpenAI ChatCompletion にリクエスト (ストリーミング)
    let client = reqwest::Client::new();
    let mut res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // 結果を蓄積する変数
    let mut full_response = String::new();

    // 5) chunk (小さい断片) を順次読み込む
    while let Some(chunk) = res.chunk().await.map_err(|e| e.to_string())? {
        // UTF-8 に変換
        let chunk_str = match std::str::from_utf8(&chunk) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // 1チャンクには複数行を含む場合があるので、改行ごとに分割
        for line in chunk_str.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // OpenAI のストリーミング形式は "data: { JSON }" の形
            if let Some(json_str) = line.strip_prefix("data: ") {
                if json_str == "[DONE]" {
                    // ストリーミング完了
                    break;
                }

                // JSONとしてパース
                let parsed: serde_json::Value = match serde_json::from_str(json_str) {
                    Ok(val) => val,
                    Err(_) => continue,
                };

                // 部分的な出力 (delta.content) を取り出す
                if let Some(content) = parsed["choices"][0]["delta"]["content"].as_str() {
                    // 部分文字列を full_response に追記
                    full_response.push_str(content);

                    // 6) フロントエンドに部分的な文字列を送信 (event: "token")
                    let _ = window.emit("token", content.to_string());
                }
            }
        }
    }

    // 7) 応答を DB に保存（全チャンク受信完了後）
    let now = Utc::now().naive_utc();
    let new_chat = NewChatHistory {
        session_id: &input_session_id,
        question: &message,
        answer: &full_response,
        created_at: now,
    };
    diesel::insert_into(chat_histories)
        .values(&new_chat)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    // 8) ChatResponse を返す
    Ok(ChatResponse {
        response: full_response,
        created_at: now.to_string(),
    })
}
