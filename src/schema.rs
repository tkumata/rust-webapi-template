// @generated automatically by Diesel CLI.

diesel::table! {
    user_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_tokens,
    users,
);
