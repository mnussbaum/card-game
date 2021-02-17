use std::fmt;

use diesel::Queryable;
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::Serialize;

use crate::schema::cards;

#[derive(Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_enum_suit"]
#[PgType = "card_enum_suit"]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_enum_rank"]
#[PgType = "card_enum_rank"]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

#[derive(Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_enum_color"]
#[PgType = "card_enum_color"]
pub enum Color {
    Black,
    Red,
}

#[derive(GraphQLObject, Serialize, Identifiable, Queryable)]
#[table_name = "cards"]
pub struct Card {
    pub id: i32,
    pub rank_numeric: Option<i32>,
    pub rank_text: Option<Rank>,
    pub rank_symbol: Option<String>,
    pub suit_symbol: Option<String>,
    pub suit_text: Option<Suit>,
    pub suit_color: Option<Color>,
    pub unicode_char: String,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.unicode_char)
    }
}
