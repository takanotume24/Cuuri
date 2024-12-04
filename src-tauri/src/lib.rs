use std::fs;
use std::io::Write;

mod app_type;
mod config;
mod database;
mod generate_session_id;
mod get_available_models;
mod get_chat_history;
mod get_chat_history_by_session;
mod get_chatgpt_response;
mod get_config;
mod get_db_connection;
mod get_default_model;
mod get_openai_api_key;
mod get_session_id_list;
mod models;
mod schema;
mod set_openai_api_key;

use database::Database;
use generate_session_id::generate_session_id;
use get_available_models::get_available_models;
use get_chat_history::get_chat_history;
use get_chat_history_by_session::get_chat_history_by_session;
use get_chatgpt_response::get_chatgpt_response;
use get_default_model::get_default_model;
use get_openai_api_key::get_openai_api_key;
use get_session_id_list::get_session_id_list;
use set_openai_api_key::set_openai_api_key;

fn shutdown(database: &Database) {
    database
        .checkpoint()
        .expect("Failed to checkpoint database");
}

pub fn run() {
    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".cuuri/config.toml");

    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories for config file");
        }
        let mut file = fs::File::create(&config_path).expect("Failed to create config file");

        writeln!(file, "openai_api_key = \"\"").expect("Failed to write default API key");
        writeln!(file, "default_model = \"gpt-3.5-turbo\"").expect("Failed to write default model");

        println!("Configuration file created at {:?}", config_path);
    }

    let mut db_path = dirs::home_dir().expect("Failed to get home directory");
    db_path.push(".cuuri/chat.db");

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories for database file");
    }

    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

    let database = Database::new(database_url);
    database.initialize();

    tauri::Builder::default()
        .manage(database.clone())
        .invoke_handler(tauri::generate_handler![
            get_chatgpt_response,
            get_chat_history,
            generate_session_id,
            get_available_models,
            get_default_model,
            set_openai_api_key,
            get_openai_api_key,
            get_chat_history_by_session,
            get_session_id_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    shutdown(&database);
}
