use crate::get_config::get_config;

#[tauri::command]
pub async fn get_openai_api_key() -> Result<String, String> {
    let config = get_config()
        .await
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    Ok(config.openai_api_key)
}
