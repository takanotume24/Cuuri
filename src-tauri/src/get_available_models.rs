#[tauri::command]
pub async fn get_available_models(api_key: String) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to fetch models: HTTP {}", res.status()));
    }

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let mut models: Vec<String> = json["data"]
        .as_array()
        .ok_or("Invalid response format")?
        .iter()
        .filter_map(|model| model["id"].as_str().map(|s| s.to_string()))
        .collect();

    models.sort();

    Ok(models)
}
