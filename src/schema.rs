table! {
    games (id) {
        id -> Int4,
        player_turn_index -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    games_users (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_uuid -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(games_users -> games (game_id));
joinable!(games_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    games_users,
    users,
);
