use dirs;
use std::fs;
use std::io;

pub fn get_database_path() -> Result<String, io::Error> {
    // ホームディレクトリの取得
    let mut db_path = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Failed to get home directory"))?;
    db_path.push(".cuuri/chat.db");

    // 親ディレクトリの作成
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // データベースURLの生成
    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

    Ok(database_url) // 成功時はOkを返す
}
