mod app_type;
mod config;
mod establish_connection;
mod generate_session_id;
mod get_available_models;
mod get_chat_history;
mod get_chat_history_by_session;
mod get_config;
mod get_database_path;
mod get_default_model;
mod get_openai_api_key;
mod get_session_id_list;
mod init_config_file;
mod models;
mod run_migrations;
mod schema;
mod set_openai_api_key;
mod stream_chatgpt_response;

use establish_connection::establish_connection;
use generate_session_id::generate_session_id;
use get_available_models::get_available_models;
use get_chat_history::get_chat_history;
use get_chat_history_by_session::get_chat_history_by_session;
use get_database_path::get_database_path;
use get_default_model::get_default_model;
use get_openai_api_key::get_openai_api_key;
use get_session_id_list::get_session_id_list;
use init_config_file::init_config_file;
use run_migrations::run_migrations;
use set_openai_api_key::set_openai_api_key;
use std::env;
use stream_chatgpt_response::stream_chatgpt_response;

pub fn run() {
    if let Err(e) = init_config_file() {
        eprintln!("Failed to initialize config file: {}", e);
        return;
    }

    let database_path = match get_database_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get database path: {}", e);
            return;
        }
    };

    let mut connection = match establish_connection(&database_path) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to establish connection: {}", e);
            return;
        }
    };

    if let Err(e) = run_migrations(&mut connection) {
        eprintln!("Failed to run migrations: {}", e);
        return;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_chat_history,
            generate_session_id,
            get_available_models,
            get_default_model,
            set_openai_api_key,
            get_openai_api_key,
            get_chat_history_by_session,
            get_session_id_list,
            stream_chatgpt_response,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
