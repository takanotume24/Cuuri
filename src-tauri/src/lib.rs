use dirs;
use std::env;
use std::fs;
use std::io::Write;

mod commands;
mod config;
mod database;
mod generate_session_id;
mod get_available_models;
mod get_chat_history;
mod get_config;
mod get_db_connection;
mod get_default_model;
mod get_openai_api_key;
mod models;
mod schema;
mod set_openai_api_key;

use commands::*;
use config::Config;
use database::Database;
use generate_session_id::generate_session_id;
use get_available_models::get_available_models;
use get_chat_history::get_chat_history;
use get_default_model::get_default_model;
use get_openai_api_key::get_openai_api_key;
use set_openai_api_key::set_openai_api_key;

fn shutdown(database: &Database) {
    database
        .checkpoint()
        .expect("Failed to checkpoint database");
}

pub fn run() {
    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".chauri/config.toml");

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
    db_path.push(".chauri/chat.db");

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories for database file");
    }

    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

    let database = Database::new(database_url);
    database.initialize();

    tauri::Builder::default()
        .manage(database.clone())
        .invoke_handler(tauri::generate_handler![
            chat_gpt,
            get_chat_history,
            generate_session_id,
            get_available_models,
            get_default_model,
            set_openai_api_key,
            get_openai_api_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    shutdown(&database);
}
