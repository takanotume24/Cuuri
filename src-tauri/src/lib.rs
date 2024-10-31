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
use chrono::NaiveDateTime;

mod schema {
    diesel::table! {
        chat_history (id) {
            id -> Integer,
            session_id -> Text,
            question -> Text,
            answer -> Text,
            created_at -> Timestamp,
        }
    }
}

mod models {
    use super::schema::chat_history;
    use diesel::prelude::*;
    use chrono::NaiveDateTime;

    #[derive(Queryable, Insertable)]
    #[diesel(table_name = chat_history)]
    pub struct ChatHistory {
        pub id: i32,
        pub session_id: String,
        pub question: String,
        pub answer: String,
        pub created_at: NaiveDateTime,
    }

    #[derive(Insertable)]
    #[diesel(table_name = chat_history)]
    pub struct NewChatHistory<'a> {
        pub session_id: &'a str,
        pub question: &'a str,
        pub answer: &'a str,
        pub created_at: NaiveDateTime, // Changed to NaiveDateTime
    }
}

use models::{ChatHistory, NewChatHistory};
use schema::chat_history::dsl::*;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

struct ApiKey(String);

#[derive(Clone)]
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
                answer TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
            )",
        )
        .execute(&mut conn)
        .expect("Failed to create table");
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
    model: String,
    state: State<'_, ApiKey>,
    db: State<'_, Database>,
) -> Result<String, String> {
    let mut conn = db.pool.get().map_err(|e| e.to_string())?;

    // Retrieve the entire chat history for the session, ordered by created_at
    let session_history = chat_history
        .filter(session_id.eq(input_session_id.clone()))
        .order(created_at.asc())
        .load::<ChatHistory>(&mut conn)
        .map_err(|e| e.to_string())?;

    // Construct the message list for the API request in correct order
    let mut messages = Vec::new();
    for entry in session_history {
        messages.push(json!({"role": "user", "content": entry.question}));
        messages.push(json!({"role": "assistant", "content": entry.answer}));
    }
    // Add the new user message to the end of the list
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

    let now = chrono::Utc::now().naive_utc();
    let new_chat = NewChatHistory {
        session_id: input_session_id.as_str(),
        question: message.as_str(),
        answer: response,
        created_at: now, // Directly use NaiveDateTime
    };

    diesel::insert_into(chat_history)
        .values(&new_chat)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(response.to_string())
}


#[tauri::command]
async fn get_chat_history(db: State<'_, Database>) -> Result<Vec<HashMap<String, String>>, String> {
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
fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

#[tauri::command]
async fn get_available_models(state: State<'_, ApiKey>) -> Result<Vec<String>, String> {
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

    // Sort the models alphabetically
    models.sort();

    Ok(models)
}

#[tauri::command]
fn get_default_model() -> Result<String, String> {
    dotenv().ok();
    match env::var("DEFAULT_MODEL") {
        Ok(default_model) => Ok(default_model),
        Err(e) => Err(format!("Failed to get DEFAULT_MODEL: {}", e)),
    }
}

fn shutdown(database: &Database) {
    database
        .checkpoint()
        .expect("Failed to checkpoint database");
}

pub fn run() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database = Database::new(database_url);
    database.initialize();

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .manage(database.clone())
        .invoke_handler(tauri::generate_handler![
            chat_gpt,
            get_chat_history,
            generate_session_id,
            get_available_models,
            get_default_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    shutdown(&database);
}
