use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use sqlx::migrate::MigrateDatabase;
use sqlx::{sqlite::Sqlite, Row, SqlitePool};
use std::env;
use tauri::async_runtime::block_on;
use tauri::State;
use std::collections::HashMap;

struct ApiKey(String);

struct Database(SqlitePool);

impl Database {
    fn pool(&self) -> &SqlitePool {
        &self.0
    }
}

#[tauri::command]
async fn chat_gpt(
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

    let pool = db.pool();
    sqlx::query("INSERT INTO chat_history (question, answer) VALUES (?, ?)")
        .bind(&message)
        .bind(&response)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response.to_string())
}

#[tauri::command]
async fn get_chat_history(db: State<'_, Database>) -> Result<Vec<HashMap<String, String>>, String> {
    let pool = db.pool();
    let rows = sqlx::query("SELECT question, answer FROM chat_history")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let question: String = row.get("question");
            let answer: String = row.get("answer");
            let mut map = HashMap::new();
            map.insert("question".to_string(), question);
            map.insert("answer".to_string(), answer);
            map
        })
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}


pub fn run() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 非同期の初期化をblock_onで実行
    block_on(async {
        // データベースが存在しない場合に作成
        if !Sqlite::database_exists(&database_url)
            .await
            .unwrap_or(false)
        {
            println!("Creating database at {}", database_url);
            Sqlite::create_database(&database_url)
                .await
                .expect("Failed to create database");
        } else {
            println!("Database already exists at {}", database_url);
        }
    });

    let db_pool = block_on(async {
        let pool = SqlitePool::connect(&database_url).await.expect("Failed to create database pool");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS chat_history (id INTEGER PRIMARY KEY, question TEXT, answer TEXT)",
        )
        .execute(&pool)
        .await
        .expect("Failed to create table");

        pool
    });

    tauri::Builder::default()
        .manage(ApiKey(api_key))
        .manage(Database(db_pool))
        .invoke_handler(tauri::generate_handler![chat_gpt, get_chat_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
