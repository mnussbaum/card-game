table! {
    use diesel::sql_types::*;
    use crate::deck::records::*;

    card_groups (id) {
        id -> Int4,
        created_at -> Timestamp,
        name -> Varchar,
        initial_size -> Nullable<Int4>,
        layout -> Card_group_enum_layout,
        visibility -> Card_group_enum_visibility,
        user_id -> Nullable<Int4>,
        game_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::records::*;

    card_groups_cards (id) {
        id -> Int4,
        created_at -> Timestamp,
        card_id -> Int4,
        card_group_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::records::*;

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
    use crate::deck::records::*;

    games (id) {
        id -> Int4,
        player_turn_index -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::records::*;

    games_users (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::deck::records::*;

    users (id) {
        id -> Int4,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(card_groups -> games (game_id));
joinable!(card_groups -> users (user_id));
joinable!(card_groups_cards -> card_groups (card_group_id));
joinable!(card_groups_cards -> cards (card_id));
joinable!(games_users -> games (game_id));
joinable!(games_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    card_groups,
    card_groups_cards,
    cards,
    games,
    games_users,
    users,
);
