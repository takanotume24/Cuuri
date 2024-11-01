use super::schema::chat_history;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = chat_history)]
pub struct ChatHistory {
    pub id: i32,
    pub session_id: String,
    pub question: String,
    pub answer: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = chat_history)]
pub struct NewChatHistory<'a> {
    pub session_id: &'a str,
    pub question: &'a str,
    pub answer: &'a str,
    pub created_at: NaiveDateTime,
}
