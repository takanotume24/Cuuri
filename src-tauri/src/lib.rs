use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use tauri::State;
use uuid::Uuid;

mod schema {
    diesel::table! {
        chat_history (id) {
            id -> Integer,
            session_id -> Text,
            question -> Text,
            answer -> Text,
        }
    }
}

mod models {
    use super::schema::chat_history;
    use diesel::prelude::*;

    #[derive(Queryable, Insertable)]
    #[diesel(table_name = chat_history)]
    pub struct ChatHistory {
        pub id: i32,
        pub session_id: String,
        pub question: String,
        pub answer: String,
    }

    #[derive(Insertable)]
    #[diesel(table_name = chat_history)]
    pub struct NewChatHistory<'a> {
        pub session_id: &'a str,
        pub question: &'a str,
        pub answer: &'a str,
    }
}

use models::{ChatHistory, NewChatHistory};
use schema::chat_history::dsl::*;

struct ApiKey(String);

struct Database(String);

impl Database {
    fn get_connection(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.0).expect("Failed to connect to database")
    }

    fn initialize(&self) {
        let mut conn = self.get_connection();
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS chat_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                question TEXT NOT NULL,
                answer TEXT NOT NULL
            )"
        ).execute(&mut conn).expect("Failed to create table");
    }
}

#[tauri::command]
async fn chat_gpt(
    input_session_id: String, // ここで変数名を変更
    message: String,
    state: State<'_, ApiKey>,
    db: State<'_, Database>,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": message}],
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.0))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let response = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "No response from API".to_string())?;

    let mut conn = db.get_connection();
    let new_chat = NewChatHistory {
        session_id: input_session_id.as_str(), // 修正: 正しい変数名を使用
        question: message.as_str(),
        answer: response,
    };

    diesel::insert_into(chat_history)
        .values(&new_chat)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(response.to_string())
}

#[tauri::command]
async fn get_chat_history(
    db: State<'_, Database>,
) -> Result<Vec<HashMap<String, String>>, String> {
    let mut conn = db.get_connection();

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
            map
        })
        .collect();

    Ok(rows)
}

#[tauri::command]
fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn run() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database = Database(database_url.clone());
    database.initialize(); // テーブルの自動作成

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            chat_gpt,
            get_chat_history,
            generate_session_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
