use crate::app_type::ChatResponse;
use crate::establish_connection::establish_connection;
use crate::get_database_path::get_database_path;
use crate::models::{ChatHistory, NewChatHistory};
use crate::schema::chat_histories::dsl::*;
use chrono::Utc;
use diesel::prelude::*;
use tauri::{Emitter, Window};

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImageArgs,
    ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, ImageDetail, ImageUrlArgs,
};
use async_openai::Client;
use futures_util::StreamExt; // for `while let Some(...) = stream.next().await`
use std::error::Error;

/// Fetch session history from the DB
fn fetch_session_history(
    conn: &mut SqliteConnection,
    input_session_id: &str,
) -> Result<Vec<ChatHistory>, String> {
    chat_histories
        .filter(session_id.eq(input_session_id))
        .order(created_at.asc())
        .load::<ChatHistory>(conn)
        .map_err(|e| e.to_string())
}

/// Convert a list of `ChatHistory` records into a Vec of async_openai messages.
/// （従来の履歴からメッセージを生成する部分は変更なし）
fn build_messages_from_history(
    session_history: &[ChatHistory],
) -> Vec<ChatCompletionRequestMessage> {
    let mut messages = Vec::new();

    for entry in session_history {
        // ユーザーの発言
        messages.push(ChatCompletionRequestMessage::User(
            entry.question.clone().into(),
        ));

        // アシスタントの発言
        messages.push(ChatCompletionRequestMessage::Assistant(
            entry.answer.clone().into(),
        ));
    }

    messages
}

/// Build a single "user" message from text plus optional base64 images,
/// sending images as separate content parts.
/// base64_images 内の各文字列は、実際は画像の base64 エンコードデータ（※仕様に合わせてフィールド名等調整）。
fn build_user_message(
    message: &str,
    base64_images: Option<Vec<String>>,
) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
    // 複数のパートで構成されるメッセージ内容を保持するベクター
    let mut content_parts = Vec::new();

    // まずテキストパートを追加
    let text_part = ChatCompletionRequestMessageContentPartTextArgs::default()
        .text(message)
        .build()?
        .into();
    content_parts.push(text_part);

    // オプションで渡された base64 エンコード画像をそれぞれ画像パートとして追加
    if let Some(images) = base64_images {
        for base64_img in images {
            let image_part = ChatCompletionRequestMessageContentPartImageArgs::default()
                .image_url(
                    ImageUrlArgs::default()
                        .url(base64_img) // ここに base64 エンコードされた画像データを設定
                        .detail(ImageDetail::High)
                        .build()?,
                )
                .build()?
                .into();
            content_parts.push(image_part);
        }
    }

    // ユーザーメッセージとしてビルド
    let user_message = ChatCompletionRequestUserMessageArgs::default()
        .content(content_parts)
        .build()?
        .into();

    Ok(user_message)
}

/// Store the question/answer pair back into the DB.
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

/// The main Tauri command function that streams a ChatGPT response using async-openai.
#[tauri::command]
pub async fn stream_chatgpt_response(
    window: Window,
    input_session_id: String,
    message: String,
    base64_images: Option<Vec<String>>,
    model: String,
    api_key: String,
) -> Result<ChatResponse, String> {
    // Step 1: Open DB connection
    let database_path = get_database_path().map_err(|e| e.to_string())?;
    let mut conn = establish_connection(&database_path).map_err(|e| e.to_string())?;

    // Step 2: Fetch session history and build messages
    let session_history = fetch_session_history(&mut conn, &input_session_id)?;
    let mut messages = build_messages_from_history(&session_history);

    // Step 3: Add current user message using the new multi-part implementation
    let user_msg = build_user_message(&message, base64_images).map_err(|e| e.to_string())?;
    messages.push(user_msg);

    // Step 4: Build the chat request, enabling streaming
    let request = CreateChatCompletionRequestArgs::default()
        .model(model.clone())
        .messages(messages)
        .stream(true)
        .build()
        .map_err(|e| format!("Failed to build ChatCompletion request: {}", e))?;

    // Step 5: Create an async-openai client
    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

    // Step 6: Start streaming
    let mut response_stream = client
        .chat()
        .create_stream(request)
        .await
        .map_err(|e| format!("Failed to create stream: {}", e))?;

    // Step 7: Process streaming response, collecting the full answer
    let mut full_response = String::new();
    while let Some(resp_chunk) = response_stream.next().await {
        match resp_chunk {
            Ok(chunk) => {
                if let Some(first_choice) = chunk.choices.first() {
                    if let Some(delta_content) = &first_choice.delta.content {
                        full_response.push_str(delta_content);
                        // Emit partial tokens to the front-end
                        let _ = window.emit("token", delta_content.clone());
                    }
                }
            }
            Err(err) => return Err(format!("Stream error: {}", err)),
        }
    }

    // Step 8: Store the Q&A pair into the DB
    store_response_to_db(&mut conn, &input_session_id, &message, &full_response)?;

    // Step 9: Return final result
    let now = Utc::now().naive_utc();
    Ok(ChatResponse {
        response: full_response,
        created_at: now.to_string(),
    })
}
