use std::fmt;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

use crate::db::PooledConnection;
use crate::errors::ServiceResult;
use crate::game::record::GameRecord;
use crate::schema::{card_groups, card_groups_cards, cards};
use crate::user::model::User;

#[derive(Clone, Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_enum_suit"]
#[PgType = "card_enum_suit"]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, Eq, Hash, DbEnum, GraphQLEnum, Deserialize, Serialize,
)]
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

#[derive(Clone, Debug, PartialEq, DbEnum, GraphQLEnum, Serialize)]
#[DieselType = "Card_enum_color"]
#[PgType = "card_enum_color"]
pub enum Color {
    Black,
    Red,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CardValue {
    Wild,
    Numeric(usize),
}

#[derive(Clone, GraphQLObject, Serialize, Identifiable, Queryable)]
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

const COVERED_CARD_ID: i32 = 0;

impl Card {
    pub fn belonging_to_card_group(
        connection: &PooledConnection,
        card_group_record: &CardGroupRecord,
    ) -> ServiceResult<Vec<Self>> {
        Ok(cards::table
            .inner_join(card_groups_cards::table.inner_join(card_groups::table))
            .filter(card_groups::id.eq(card_group_record.id))
            .select(cards::all_columns)
            .order(card_groups_cards::id)
            .load::<Self>(connection)?)
    }

    pub fn repeat_covered_card(
        connection: &PooledConnection,
        card_count: usize,
    ) -> ServiceResult<Vec<Self>> {
        let covered_card = cards::table
            .find(COVERED_CARD_ID)
            .select(cards::all_columns)
            .load::<Self>(connection)?
            .pop()
            .expect("Couldn't find covered card in DB");

        Ok(std::iter::repeat(covered_card)
            .take(card_count)
            .collect::<Vec<Self>>())
    }
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

#[derive(Clone, Debug, PartialEq, DbEnum, GraphQLEnum, Deserialize, Serialize)]
#[DieselType = "Card_group_enum_layout"]
#[PgType = "card_grou_enum_layout"]
pub enum CardGroupLayout {
    Pile,
    Spread,
}

#[derive(Clone, Debug, PartialEq, DbEnum, GraphQLEnum, Deserialize, Serialize)]
#[DieselType = "Card_group_enum_visibility"]
#[PgType = "card_grou_enum_visibility"]
pub enum CardGroupVisibility {
    FaceDown,
    FaceUp,
    TopFaceUpRestFaceDown,
    VisibleToOwner,
}

#[derive(Clone, Debug, Identifiable, Queryable, Associations)]
#[table_name = "card_groups"]
#[belongs_to(User)]
#[belongs_to(GameRecord, foreign_key = "game_id")]
pub struct CardGroupRecord {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub initial_size: Option<i32>,
    pub layout: CardGroupLayout,
    pub visibility: CardGroupVisibility,
    pub user_id: Option<i32>,
    pub game_id: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CardGroupDescription {
    pub name: String,
    pub initial_size: Option<i32>,
    pub layout: CardGroupLayout,
    pub visibility: CardGroupVisibility,
}

impl CardGroupRecord {
    pub fn find_by_game_record_and_user(
        connection: &PooledConnection,
        game_record: &GameRecord,
        user: &User,
    ) -> ServiceResult<Vec<Self>> {
        Ok(card_groups::table
            .filter(card_groups::game_id.eq(game_record.id))
            .filter(card_groups::user_id.eq(user.id))
            .select(card_groups::all_columns)
            .load::<Self>(connection)?)
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(CardGroupRecord foreign_key = "card_group_id")]
#[belongs_to(Card)]
#[table_name = "card_groups_cards"]
pub struct CardGroupCard {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub card_id: i32,
    pub card_group_id: i32,
}
