use serde::Serialize;

#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct RawDatabaseChatEntry {
    pub session_id: String,
    pub question: String,
    pub answer: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct SessionId(pub String);
