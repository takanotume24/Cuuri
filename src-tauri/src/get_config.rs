use crate::config::Config;
use std::path::PathBuf;

pub async fn get_config() -> Result<Config, String> {
    // Attempt to get the home directory
    let mut config_path: PathBuf = dirs::home_dir().ok_or("Failed to get home directory".to_string())?;
    
    config_path.push(".cuuri/config.toml");

    // Check if the config file exists
    if !config_path.exists() {
        return Err("Configuration file does not exist".to_string());
    }

    // Convert the path to a string and handle possible error
    let config_path_str = config_path.to_str().ok_or("Failed to convert config path to string".to_string())?;
    
    // Load the configuration from file and handle potential errors
    let config = Config::from_file(config_path_str)
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    Ok(config)
}