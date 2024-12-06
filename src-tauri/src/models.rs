use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::chat_histories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChatHistory {
    pub id: i32,
    pub session_id: String,
    pub question: String,
    pub answer: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::chat_histories)]
pub struct NewChatHistory<'a> {
    pub session_id: &'a String,
    pub question: &'a String,
    pub answer: &'a String,
    pub created_at: NaiveDateTime,
}
