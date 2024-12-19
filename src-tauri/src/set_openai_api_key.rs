use crate::config::Config;
use std::fs;
use std::path::PathBuf;

#[tauri::command]
pub async fn set_openai_api_key(api_key: String) -> Result<(), String> {
    // Return an error if the API key is empty
    if api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    // Attempt to get the home directory, returning an error if it fails
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Failed to get home directory".to_string())?;

    let mut config_path = PathBuf::from(home_dir);
    config_path.push(".cuuri/config.toml");

    // Attempt to load the configuration from the file
    let mut config = Config::from_file(config_path.to_str().ok_or_else(|| "Invalid config path".to_string())?)
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    config.openai_api_key = api_key;

    // Serialize the config and check for errors
    let content = toml::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    // Write the serialized config to a file
    fs::write(&config_path, content)
        .map_err(|e| format!("Failed to write to config file: {}", e))?;

    Ok(())
}