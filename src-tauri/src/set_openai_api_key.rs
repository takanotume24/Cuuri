use crate::config::Config;
use std::fs;

#[tauri::command]
pub async fn set_openai_api_key(api_key: String) -> Result<(), String> {
    // Return an error if the API key is empty
    if api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".cuuri/config.toml");

    let mut config = Config::from_file(config_path.to_str().unwrap())
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    config.openai_api_key = api_key;

    let content =
        toml::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, content)
        .map_err(|e| format!("Failed to write to config file: {}", e))?;

    Ok(())
}