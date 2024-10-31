use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
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

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

struct ApiKey(String);

#[derive(Clone)]  // Implement the Clone trait for Database
struct Database {
    pool: Arc<Pool>,
}

impl Database {
    fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database {
            pool: Arc::new(pool),
        }
    }

    fn initialize(&self) {
        let mut conn = self.pool.get().expect("Failed to get connection");
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS chat_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                question TEXT NOT NULL,
                answer TEXT NOT NULL
            )"
        ).execute(&mut conn).expect("Failed to create table");
    }

    fn checkpoint(&self) -> Result<(), String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::sql_query("PRAGMA wal_checkpoint(TRUNCATE)")
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[tauri::command]
async fn chat_gpt(
    input_session_id: String,
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

    let mut conn = db.pool.get().map_err(|e| e.to_string())?;
    let new_chat = NewChatHistory {
        session_id: input_session_id.as_str(),
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
            map
        })
        .collect();

    Ok(rows)
}

#[tauri::command]
fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

fn shutdown(database: &Database) {
    database.checkpoint().expect("Failed to checkpoint database");
}

pub fn run() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database = Database::new(database_url);
    database.initialize();

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .manage(database.clone()) // Clone to use in shutdown
        .invoke_handler(tauri::generate_handler![
            chat_gpt,
            get_chat_history,
            generate_session_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    shutdown(&database);
}
