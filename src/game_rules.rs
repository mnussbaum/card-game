use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::card_deck::{CardGroup, CardRank, CardValue};
use crate::game_state::GameState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RelativeCard {
    LastPlayedCard,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayableCardConstraint {
    MinimumPlayableCard(RelativeCard),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardMove {
    card_group_name_source: CardGroupId,
    card_group_name_dest: CardGroupId,

    #[serde(default)]
    conditions: Vec<Condition>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardSwap {
    first_card_group_name: CardGroupId,
    second_card_group_name: CardGroupId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnMove {
    offset_from_current_player: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Verb {
    ConstrainPlayableCards(PlayableCardConstraint),
    MoveCards(CardMove),
    MoveNextTurn(TurnMove),
    SwapCards(CardSwap),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CardGroupOwner {
    // TODO: Allow player lookup by name. Right now it's just "communal_cards" that's allowed
    Name(String),
    RelativePlayer { offset_from_current_player: usize },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardGroupId {
    pub owner: CardGroupOwner,
    pub name: String,
}

const COMMUNAL_CARDS: &str = "communal_cards";

impl CardGroupId {
    // TODO: Add an error type
    fn card_group<'a>(&self, game_state: &'a GameState) -> Result<&'a CardGroup, String> {
        match &self.owner {
            CardGroupOwner::Name(owner_name) => {
                if owner_name == COMMUNAL_CARDS {
                    if let Some(card_group) = game_state.communal_cards.get(&self.name) {
                        Ok(card_group)
                    } else {
                        Err(format!(
                            "Communal card group name doesn't match anything. Given: {}. Available: {}",
                            self.name,
                            game_state
                                .communal_cards
                                .keys()
                                .map(|k| k.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                        )
                        .into())
                    }
                } else {
                    Err(format!(
                        "Invalid card group owner name. Right now only 'communal_cards' is supported. Given: {}",
                        owner_name,
                    ).into())
                }
            }

            CardGroupOwner::RelativePlayer {
                offset_from_current_player,
            } => {
                if let Some(player) =
                    game_state.offset_from_current_player(*offset_from_current_player)
                {
                    if let Some(card_group) = player.hand.get(&self.name) {
                        Ok(card_group)
                    } else {
                        Err(format!(
                            "Player hand card group name doesn't match anything. Given: {}. Available: {}",
                            self.name,
                            player
                                .hand
                                .keys()
                                .map(|k| k.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                        )
                        .into())
                    }
                } else {
                    panic!(format!(
                        "Invalid player index. Given: {}. Player count: {}",
                        offset_from_current_player,
                        game_state.players.len(),
                    ));
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Operator {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl Operator {
    pub fn compare(&self, lhs: usize, rhs: usize) -> bool {
        match self {
            Operator::Equal => lhs == rhs,
            Operator::GreaterThan => lhs > rhs,
            Operator::GreaterThanOrEqual => lhs >= rhs,
            Operator::LessThan => lhs < rhs,
            Operator::LessThanOrEqual => lhs <= rhs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Condition {
    CardGroupSize {
        card_group_name: CardGroupId,
        operator: Operator,
        compare_to: usize,
    },
    CardsMustBeSameRank,
    MustBeInTurnRange {
        min: Option<usize>,
        max: Option<usize>,
    },
}

impl Condition {
    fn met(&self, game_state: &GameState) -> Result<bool, String> {
        match self {
            Condition::CardGroupSize {
                card_group_name,
                operator,
                compare_to,
            } => {
                let card_group = card_group_name.card_group(game_state)?;
                Ok(operator.compare(card_group.cards.len(), *compare_to))
            }
            // TODO: This feels like it shouldn't be the same type of condition
            Condition::CardsMustBeSameRank => Ok(true),
            Condition::MustBeInTurnRange { min, max } => {
                if let Some(min) = min {
                    if game_state.turn_count < *min {
                        return Ok(false);
                    }
                }
                if let Some(max) = max {
                    if game_state.turn_count > *max {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub description: String,
    verb: Verb,

    #[serde(default)]
    conditions: Vec<Condition>,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Action {
    fn all_conditions_met(&self, game_state: &GameState) -> Result<bool, String> {
        for condition in self.conditions.iter() {
            if !condition.met(game_state)? {
                return Ok(false);
            }
        }

        return Ok(true);
    }

    pub fn execute(&self, game_state: &mut GameState) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurnRange {
    Bounded { min: usize, max: usize },
    LowerBounded { min: usize },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardDescription {
    pub value: CardValue,
    pub consequences: Vec<Action>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerCount {
    AllPlayers,
    AllButOnePlayer,
    SomePlayers(usize),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndingCondition {
    pub player_count: PlayerCount,
    pub card_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub player_hand: HashMap<String, CardGroup>,
    pub communal_cards: HashMap<String, CardGroup>,
    pub cards: HashMap<CardRank, CardDescription>,
    pub turn_actions: Vec<Action>,
    pub ending_conditions: Vec<EndingCondition>,
}

impl GameRules {
    pub fn available_actions(&self, game_state: &GameState) -> Result<Vec<&Action>, String> {
        self.turn_actions.iter().try_fold(
            Vec::new(),
            |mut available_actions, action| -> Result<Vec<&Action>, String> {
                if action.all_conditions_met(game_state)? {
                    available_actions.push(action);
                }

                Ok(available_actions)
            },
        )
    }
}
