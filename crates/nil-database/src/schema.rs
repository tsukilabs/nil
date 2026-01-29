// @generated automatically by Diesel CLI.

diesel::table! {
    user_data (id) {
        id -> Integer,
        user -> Text,
        password -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    world_data (id) {
        id -> Text,
        password -> Nullable<Text>,
        created_by -> Integer,
        created_at -> Text,
        updated_at -> Text,
        data -> Binary,
    }
}

diesel::joinable!(world_data -> user_data (created_by));

diesel::allow_tables_to_appear_in_same_query!(user_data, world_data,);
