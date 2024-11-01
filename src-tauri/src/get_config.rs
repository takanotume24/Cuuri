use crate::config::Config;

pub async fn get_config() -> Result<Config, String> {
    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".chauri/config.toml");

    if !config_path.exists() {
        return Err("Configuration file does not exist".to_string());
    }

    let config = Config::from_file(config_path.to_str().unwrap())
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    Ok(config)
}
