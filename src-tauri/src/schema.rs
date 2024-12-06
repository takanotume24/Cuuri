// @generated automatically by Diesel CLI.

diesel::table! {
    chat_histories (id) {
        id -> Integer,
        session_id -> Text,
        question -> Text,
        answer -> Text,
        created_at -> Timestamp,
    }
}
