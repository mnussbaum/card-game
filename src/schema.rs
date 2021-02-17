table! {
    use diesel::sql_types::*;
    use crate::deck::models::*;

    cards (id) {
        id -> Int4,
        rank_numeric -> Nullable<Int4>,
        rank_text -> Nullable<Card_enum_rank>,
        rank_symbol -> Nullable<Bpchar>,
        suit_symbol -> Nullable<Bpchar>,
        suit_text -> Nullable<Card_enum_suit>,
        suit_color -> Nullable<Card_enum_color>,
        unicode_char -> Bpchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::models::*;

    games (id) {
        id -> Int4,
        player_turn_index -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::models::*;

    games_users (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::models::*;

    users (id) {
        id -> Int4,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(games_users -> games (game_id));
joinable!(games_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cards,
    games,
    games_users,
    users,
);
