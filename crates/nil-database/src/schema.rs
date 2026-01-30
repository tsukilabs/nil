// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Text,
        password -> Nullable<Text>,
        description -> Nullable<Text>,
        created_by -> Integer,
        created_at -> Text,
        updated_at -> Text,
        world_blob -> Binary,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        player_id -> Text,
        password -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(game -> user (created_by));

diesel::allow_tables_to_appear_in_same_query!(game, user,);
