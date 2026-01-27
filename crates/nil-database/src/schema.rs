// @generated automatically by Diesel CLI.

diesel::table! {
    user_data (id) {
        id -> Integer,
        user -> Text,
        password -> Text,
    }
}

diesel::table! {
    world_data (id) {
        id -> Text,
        password -> Nullable<Text>,
        data -> Binary,
    }
}

diesel::allow_tables_to_appear_in_same_query!(user_data, world_data,);
