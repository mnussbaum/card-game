table! {
    games (id) {
        id -> Int4,
        player_turn_index -> Int4,
    }
}

table! {
    games_players (id) {
        id -> Int4,
        player_id -> Int4,
        game_id -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(games_players -> games (game_id));
joinable!(games_players -> players (player_id));

allow_tables_to_appear_in_same_query!(
    games,
    games_players,
    players,
);
