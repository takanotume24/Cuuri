use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use sqlx::{sqlite::Sqlite, Row, SqliteConnection, Connection, migrate::MigrateDatabase};
use std::collections::HashMap;
use std::env;
use tauri::{async_runtime::block_on, State, WindowEvent};
use uuid::Uuid;

struct ApiKey(String);

struct Database(String);

impl Database {
    async fn get_connection(&self) -> Result<SqliteConnection, sqlx::Error> {
        SqliteConnection::connect(&self.0).await
    }
}

#[tauri::command]
async fn chat_gpt(
    session_id: String,
    message: String,
    state: State<'_, ApiKey>,
    db: State<'_, Database>,
) -> Result<String, String> {
    let client = Client::new();

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
        .ok_or("No response from API".to_string())?;

    let mut conn = db.get_connection().await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO chat_history (session_id, question, answer) VALUES (?, ?, ?)")
        .bind(&session_id)
        .bind(&message)
        .bind(&response)
        .execute(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response.to_string())
}

#[tauri::command]
async fn get_chat_history(
    db: State<'_, Database>,
) -> Result<Vec<HashMap<String, String>>, String> {
    let mut conn = db.get_connection().await.map_err(|e| e.to_string())?;
    let rows = sqlx::query("SELECT session_id, question, answer FROM chat_history")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let session_id: String = row.get("session_id");
            let question: String = row.get("question");
            let answer: String = row.get("answer");
            let mut map = HashMap::new();
            map.insert("session_id".to_string(), session_id);
            map.insert("question".to_string(), question);
            map.insert("answer".to_string(), answer);
            map
        })
        .fetch_all(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
fn generate_session_id() -> String {
    Uuid::new_v4().to_string() // Using UUIDv4 for simplicity
}

pub fn run() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    block_on(async {
        if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
            println!("Creating database at {}", database_url);
            Sqlite::create_database(&database_url).await.expect("Failed to create database");
        } else {
            println!("Database already exists at {}", database_url);
        }

        let mut conn = SqliteConnection::connect(&database_url)
            .await
            .expect("Failed to connect to database for setup");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS chat_history (
                id INTEGER PRIMARY KEY, 
                session_id TEXT, 
                question TEXT, 
                answer TEXT
            )",
        )
        .execute(&mut conn)
        .await
        .expect("Failed to create table");

        // Explicitly close the connection
        drop(conn);
    });

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .manage(Database(database_url))
        .invoke_handler(tauri::generate_handler![
            chat_gpt,
            get_chat_history,
            generate_session_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
