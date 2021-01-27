use std::collections::HashMap;
use std::fmt;
use text_io::read;

use serde::{Deserialize, Serialize};

use crate::card_deck::{CardGroup, CardRank, CardValue};
use crate::game_state::GameState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CardGroupOwner {
    // TODO: Allow player lookup by name. Right now it's just "communal_cards" that's allowed
    Name(String),
    RelativePlayer { offset_from_current_player: usize },
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CardGroupId {
    owner: CardGroupOwner,
    name: Option<String>,
    first_with_cards_of: Option<Vec<String>>,
}

impl fmt::Debug for CardGroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for CardGroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(card_group_name) = &self.name {
            write!(f, "{}", card_group_name)
        } else if let Some(first_with_cards_of) = &self.first_with_cards_of {
            write!(
                f,
                "First group with cards of: {}",
                first_with_cards_of.join(", ")
            )
        } else {
            write!(f, "Expected card group ID to either have a name or a list of card groups with a first-with-cards condition, but found neither")
        }
    }
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
        // TODO: Prompt user to figure out what cards they want to move
        // See if their selection meets the card conditions
        // See if their selection is OK by the active consequences
        // If so, move them
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardSwap {
    first_card_group: CardGroupId,
    second_card_group: CardGroupId,
}

impl CardSwap {
    fn execute(&self, game_state: &mut GameState) -> Result<(), String> {
        // TODO: START HERE: prompt user to figure out what cards to swap, move it into user_input
        // module

        let mut card_to_move_from_first_to_second: Vec<isize> = Vec::new();
        let mut card_to_move_from_second_to_first: Vec<isize> = Vec::new();
        loop {
            println!(
                "Select card to move from {} into {}. Use 0-{} or -1 to finish:",
                self.first_card_group,
                self.second_card_group,
                self.first_card_group.card_group(game_state)?.cards.len(),
            );
            // TODO: Validate index is valid
            let selected_card_index: isize = read!();
            if selected_card_index == -1 {
                break;
            }

            card_to_move_from_first_to_second.push(selected_card_index);
        }

        loop {
            println!(
                "Select card to move from {} into {}. Use 0-{} or -1 to finish:",
                self.second_card_group,
                self.first_card_group,
                self.second_card_group.card_group(game_state)?.cards.len(),
            );
            // TODO: Validate index is valid
            let selected_card_index: isize = read!();
            if selected_card_index == -1 {
                break;
            }

            card_to_move_from_second_to_first.push(selected_card_index);
        }

        for card_index in card_to_move_from_first_to_second {
            let mut cards = self.first_card_group.card_group(game_state)?.cards;

            let card = cards.remove(card_index as usize);

            // pop one off and move it to the other, then do  the same to the other group
        }

        Ok(())
    }
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

    TurnCount {
        operator: Operator,
        compare_to: usize,
    },
    CardCount {
        operator: Operator,
        compare_to: usize,
        for_players: PlayerCount,
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

            Condition::TurnCount {
                operator,
                compare_to,
            } => Ok(operator.compare(game_state.turn_count, *compare_to)),

            Condition::CardCount {
                operator,
                compare_to,
                for_players,
            } => {
                let players_with_card_count =
                    game_state
                        .players
                        .iter()
                        .fold(0, |players_with_card_count, player| {
                            if operator.compare(
                                player
                                    .hand
                                    .values()
                                    .map(|hand_card_group| hand_card_group.cards.len())
                                    .sum(),
                                *compare_to,
                            ) {
                                players_with_card_count + 1
                            } else {
                                players_with_card_count
                            }
                        });

                match for_players {
                    PlayerCount::AllPlayers => {
                        Ok(players_with_card_count == game_state.players.len())
                    }
                    PlayerCount::AllButOnePlayer => {
                        Ok((game_state.players.len() - players_with_card_count) == 1)
                    }
                    PlayerCount::SomePlayers { player_count } => {
                        Ok(players_with_card_count == *player_count)
                    }
                }
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
                card_swap.execute(game_state)?;

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
    SomePlayers { player_count: usize },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnPhase {
    name: String,
    actions: Vec<Action>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnType {
    pub name: String,
    conditions: Vec<Condition>,
    turn_phases: Vec<TurnPhase>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub player_hand: HashMap<String, CardGroup>,
    pub communal_cards: HashMap<String, CardGroup>,
    pub cards: HashMap<CardRank, CardDescription>,
    pub game_flow: Vec<TurnType>,
}

impl GameRules {
    pub fn available_actions(&self, game_state: &GameState) -> Result<Vec<&Action>, String> {
        // TODO: START HERE:
        //  Need to implement game flow into actions
        //  Then implement actions and conditions

        self.game_flow.iter().try_fold(
            Vec::new(),
            |mut available_actions, turn_type| -> Result<Vec<&Action>, String> {
                for condition in turn_type.conditions.iter() {
                    if condition.met(game_state)? {
                        for turn_phase in turn_type.turn_phases.iter() {
                            for action in turn_phase.actions.iter() {
                                if action.available(game_state)? {
                                    available_actions.push(action);
                                }
                            }
                        }
                    }
                }

                return Ok(available_actions);
            },
        )
    }
}
