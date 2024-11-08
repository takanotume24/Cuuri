diesel::table! {
    chat_history (id) {
        id -> Integer,
        session_id -> Text,
        question -> Text,
        answer -> Text,
        created_at -> Timestamp,
    }
}
