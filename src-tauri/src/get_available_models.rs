use async_openai::config::OpenAIConfig;
use async_openai::Client;
use tauri::command;

#[command]
pub async fn get_available_models(api_key: String) -> Result<Vec<String>, String> {
    // Create a new OpenAI client with the provided API key
    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

    // モデル一覧を取得
    let model_list = client
        .models()
        .list()
        .await
        .map_err(|err| err.to_string())?;

    // 取得したモデル一覧からIDだけを抽出し、ベクタとして返す
    let mut models: Vec<String> = model_list.data.into_iter().map(|model| model.id).collect();

    // アルファベット順でソート
    models.sort();

    Ok(models)
}
