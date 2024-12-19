mod app_type;
mod config;
mod establish_connection;
mod generate_session_id;
mod get_available_models;
mod get_chat_history;
mod get_chat_history_by_session;
mod get_chatgpt_response;
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

use establish_connection::establish_connection;
use generate_session_id::generate_session_id;
use get_available_models::get_available_models;
use get_chat_history::get_chat_history;
use get_chat_history_by_session::get_chat_history_by_session;
use get_chatgpt_response::get_chatgpt_response;
use get_database_path::get_database_path;
use get_default_model::get_default_model;
use get_openai_api_key::get_openai_api_key;
use get_session_id_list::get_session_id_list;
use init_config_file::init_config_file;
use run_migrations::run_migrations;
use set_openai_api_key::set_openai_api_key;
use std::env;

pub fn run() {
    init_config_file();

    let database_path: String = get_database_path().unwrap();
    let mut connection: diesel::SqliteConnection = establish_connection(&database_path);
    run_migrations(&mut connection);

    tauri::Builder::default()
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
}
