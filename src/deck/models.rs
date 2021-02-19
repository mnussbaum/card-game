use std::fmt;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::Serialize;

use crate::db::PooledConnection;
use crate::errors::ServiceResult;
use crate::schema::{card_groups, card_groups_cards, cards};
use crate::user::model::User;

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

#[derive(Clone, Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_group_enum_layout"]
#[PgType = "card_grou_enum_layout"]
pub enum CardGroupLayout {
    Pile,
    Spread,
}

#[derive(Clone, Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_group_enum_visibility"]
#[PgType = "card_grou_enum_visibility"]
pub enum CardGroupVisibility {
    FaceDown,
    FaceUp,
    TopFaceUpRestFaceDown,
    VisibleToOwner,
}

#[derive(GraphQLObject, Serialize, Identifiable, Queryable)]
pub struct CardGroup {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub initial_size: i32,
    pub layout: CardGroupLayout,
    pub visibility: CardGroupVisibility,

    // The owner of a card group is either a user or a game in the case of
    // communal cards
    pub owner_type: String,
    pub owner_id: i32,
}

impl CardGroup {
    pub fn belonging_to_user(
        connection: &PooledConnection,
        user: &User,
    ) -> ServiceResult<Vec<Self>> {
        Ok(card_groups::table
            .filter(card_groups::owner_type.eq("user"))
            .filter(card_groups::owner_id.eq(user.id))
            .select(card_groups::all_columns)
            .load::<Self>(connection)?)
    }
}

#[derive(GraphQLObject, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(CardGroup)]
#[belongs_to(Card)]
#[table_name = "card_groups_cards"]
pub struct CardGroupCards {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub card_id: i32,
    pub card_group_id: i32,
}
