use std::fs;

pub fn get_database_path() -> String {
    let mut db_path = dirs::home_dir().expect("Failed to get home directory");
    db_path.push(".cuuri/chat.db");

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories for database file");
    }

    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

    return database_url;
}
