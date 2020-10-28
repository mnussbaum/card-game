use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::card_deck::{CardGroup, CardRank, CardValue};

// TODO: Start here
// x. Serialize game rule yaml to objects
// 2. Use rules to drive play
//
// First gather the actions available to a player from the defaults
// Then look at the consequences applying to the player, and use
// that to winnow out actions
// After user plays card re-eval consequences to handle if eg the player can play again
// Played cards are going to need a record of who played them

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Verb {
    MoveCards,
    MoveNextTurn,
    ConstrainPlayableCards,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RelativeCard {
    LastPlayedCard,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CardGroupOwner {
    Name(String),
    RelativePlayer { offset_from_current_player: usize },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CardGroupId {
    pub owner: CardGroupOwner,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Object {
    // This will need to prompt user for what card and how many
    CardMove {
        card_group_name_source: CardGroupId,
        card_group_name_dest: CardGroupId,
    },
    MinimumPlayableCard(RelativeCard),
    CardsMustBeSameRank,
    RelativePlayer {
        offset_from_current_player: usize,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Play {
    pub verb: Verb,
    pub object: Object,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurnRange {
    Bounded { min: usize, max: usize },
    LowerBounded { min: usize },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CardDescription {
    pub value: CardValue,
    pub consequences: Vec<Play>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerCount {
    AllPlayers,
    AllButOnePlayer,
    SomePlayers(usize),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EndingCondition {
    pub player_count: PlayerCount,
    pub card_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub player_hand: HashMap<String, CardGroup>,
    pub communal_cards: HashMap<String, CardGroup>,
    pub cards: HashMap<CardRank, CardDescription>,
    pub turn_actions: HashMap<TurnRange, Vec<Play>>,
    pub ending_conditions: Vec<EndingCondition>,
}
