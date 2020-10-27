use std::collections::HashMap;

use crate::card_deck::{CardGroup, CardRank, CardValue};

// TODO: Start here
// 1. Serialize game rule yaml to objects
// 2. Use rules to drive play
//
// First gather the actions available to a player from the defaults
// Then look at the consequences applying to the player, and use
// that to winnow out actions
// After user plays card re-eval consequences to handle if eg the player can play again
// Played cards are going to need a record of who played them

pub enum Verb {
    MoveCards,
    MoveNextTurn,
    ConstrainPlayableCards,
}

pub enum RelativeCard {
    LastPlayedCard,
}

pub struct RelativePlayerIndex {
    pub offset_from_current_player: usize,
}

pub enum CardGroupOwner {
    Name(String),
    RelativePlayer(RelativePlayerIndex),
}

pub struct CardGroupId {
    pub owner: CardGroupOwner,
    pub name: String,
}

pub enum Object {
    // This will need to prompt user for what card and how many
    CardMove {
        card_group_name_source: CardGroupId,
        card_group_name_dest: CardGroupId,
    },
    MinimumPlayableCard(RelativeCard),
    CardsMustBeSameRank,
    RelativePlayer(RelativePlayerIndex),
}

pub struct Play {
    pub verb: Verb,
    pub object: Object,
}

pub enum TurnRange {
    Bounded(std::ops::Range<usize>),
    LowerBounded(std::ops::RangeFrom<usize>),
}

pub struct CardDescription {
    pub value: CardValue,
    pub consequences: Vec<Play>,
}

pub enum PlayerCount {
    AllPlayers,
    AllButOnePlayer,
    SomePlayers(usize),
}

pub struct EndingCondition {
    pub player_count: PlayerCount,
    pub card_count: usize,
}

pub struct GameRules {
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub player_hand: HashMap<String, CardGroup>,
    pub communal_cards: HashMap<String, CardGroup>,
    pub cards: HashMap<CardRank, CardDescription>,
    pub turn_actions: HashMap<TurnRange, Vec<Play>>,
    pub ending_conditions: Vec<EndingCondition>,
}
