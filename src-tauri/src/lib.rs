use dirs;
use std::env;
use std::fs;
use std::io::Write;

mod commands;
mod config;
mod database;
mod models;
mod schema;

use commands::*;
use config::Config;
use database::{ApiKey, Database};

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

    let config =
        Config::from_file(config_path.to_str().unwrap()).expect("Failed to load configuration");

    let api_key = config.openai_api_key;

    let mut db_path = dirs::home_dir().expect("Failed to get home directory");
    db_path.push(".chauri/chat.db");

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories for database file");
    }

    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

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
            set_openai_api_key,
            get_openai_api_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    shutdown(&database);
}
