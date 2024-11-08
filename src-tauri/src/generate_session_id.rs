use uuid::Uuid;

#[tauri::command]
pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}
