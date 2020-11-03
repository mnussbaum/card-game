use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::card_deck::{CardGroup, CardRank, CardValue};
use crate::game_state::GameState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CardGroupOwner {
    // TODO: Allow player lookup by name. Right now it's just "communal_cards" that's allowed
    Name(String),
    RelativePlayer { offset_from_current_player: usize },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardGroupId {
    owner: CardGroupOwner,
    name: Option<String>,
    first_with_cards_of: Option<Vec<String>>,
}

const COMMUNAL_CARDS: &str = "communal_cards";

impl CardGroupId {
    // TODO: Add an error type
    fn card_group<'a>(&self, game_state: &'a GameState) -> Result<&'a CardGroup, String> {
        let hmm = match &self.owner {
            CardGroupOwner::Name(owner_name) => {
                if owner_name == COMMUNAL_CARDS {
                    &game_state.communal_cards
                } else {
                    return Err(format!(
                        "Invalid card group owner name. Right now only 'communal_cards' is supported. Given: {}",
                        owner_name,
                    ).into());
                }
            }

            CardGroupOwner::RelativePlayer {
                offset_from_current_player,
            } => {
                if let Some(player) =
                    game_state.offset_from_current_player(*offset_from_current_player)
                {
                    &player.hand
                } else {
                    return Err(format!(
                        "Invalid player index. Given: {}. Player count: {}",
                        offset_from_current_player,
                        game_state.players.len(),
                    ));
                }
            }
        };

        if let Some(name) = &self.name {
            if let Some(card_group) = hmm.get(name) {
                return Ok(&card_group);
            } else {
                return Err(format!(
                    "Player hand card group name doesn't match anything. Given: {}. Available: {}",
                    name,
                    hmm.keys()
                        .map(|k| k.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
                .into());
            }
        } else if let Some(first_with_cards_of) = &self.first_with_cards_of {
            for card_group_name in first_with_cards_of {
                if let Some(card_group) = hmm.get(card_group_name) {
                    if card_group.cards.len() > 0 {
                        return Ok(&card_group);
                    }
                }
            }

            return Err(format!(
                "None of the card groups had any cards in: {}",
                first_with_cards_of
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            ));
        }

        return Err(
            "Invalid card group identifier. Neither name nor 'first_with_cards_of' set".into(),
        );
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RelativeCard {
    LastPlayedCard,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayableCardConstraint {
    MinimumPlayableCard(RelativeCard),
}

// Maybe all the verbs should implement a trait?

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardMove {
    card_group_name_source: CardGroupId,
    card_group_name_dest: CardGroupId,

    #[serde(default)]
    card_conditions: Vec<CardCondition>,
}

impl CardMove {
    fn execute(&self, game_state: &mut GameState) -> Result<(), String> {
        // START HERE: TODO: Prompt user to figure out what cards they want to move
        // See if their selection meets the card conditions
        // See if their selection is OK by the active consequences
        // If so, move them
        Ok(())
    }
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
pub struct ActionDescriptions {
    action_descriptions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Verb {
    ConstrainPlayableCards(PlayableCardConstraint),
    EndPhase,
    ExcludeActions(ActionDescriptions),
    MoveCards(CardMove),
    MoveNextTurn(TurnMove),
    SwapCards(CardSwap),
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
enum CardCondition {
    CardsMustBeSameRank,
    CardsMustBeHigherRankThanLastPlayedOrWild,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Condition {
    AnyPlayedCardRank {
        equals: usize,
    },
    CardGroupSize {
        card_group_name: CardGroupId,
        operator: Operator,
        compare_to: usize,
    },
}

impl Condition {
    fn met(&self, game_state: &GameState) -> Result<bool, String> {
        match self {
            Condition::AnyPlayedCardRank { equals } => Ok(true),
            Condition::CardGroupSize {
                card_group_name,
                operator,
                compare_to,
            } => {
                let card_group = card_group_name.card_group(game_state)?;
                Ok(operator.compare(card_group.cards.len(), *compare_to))
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

    #[serde(default)]
    consequences: Vec<Action>,
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
    fn available(&self, game_state: &GameState) -> Result<bool, String> {
        for condition in self.conditions.iter() {
            if !condition.met(game_state)? {
                return Ok(false);
            }
        }

        // TODO: Check if action is ruled out by any of the active consequences

        return Ok(true);
    }

    pub fn execute(&self, game_state: &mut GameState) -> Result<(), String> {
        println!("{:?}", &self.verb);
        match &self.verb {
            Verb::MoveCards(card_move) => {
                card_move.execute(game_state)?;

                Ok(())
            }
            Verb::SwapCards(card_swap) => {
                // card_swap.execute(game_state)?;

                Ok(())
            }
            _ => panic!("AHHHHHHHHHHHHHHHHHHHHHHHH"),
        }
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerCount {
    AllPlayers,
    AllButOnePlayer,
    SomePlayers(usize),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EndingCondition {
    TurnCount {
        count: usize,
    },
    CardCount {
        count: usize,
        for_players: PlayerCount,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnPhase {
    name: String,
    actions: Vec<Action>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnDescription {
    pub name: String,
    until: Vec<EndingCondition>,
    turn_phases: Vec<TurnPhase>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub player_hand: HashMap<String, CardGroup>,
    pub communal_cards: HashMap<String, CardGroup>,
    pub cards: HashMap<CardRank, CardDescription>,
    pub game_flow: Vec<TurnDescription>,
}

impl GameRules {
    pub fn available_actions(&self, game_state: &GameState) -> Result<Vec<&Action>, String> {
        // self.turn_actions.iter().try_fold(
        //     Vec::new(),
        //     |mut available_actions, action| -> Result<Vec<&Action>, String> {
        //         if action.available(game_state)? {
        //             available_actions.push(action);
        //         }
        //
        //         Ok(available_actions)
        //     },
        // )
        Ok(Vec::new())
    }
}
