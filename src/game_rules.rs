use std::collections::HashMap;
use std::fmt;

use crate::deck::graphql::CardGroup;
use crate::deck::records::{Card, CardGroupDescription, CardValue, Rank};
use crate::errors::ServiceResult;
use crate::game::graphql::Game;
use serde::{Deserialize, Serialize};

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

// TODO: DRY _mut and not mut method definitions

// impl CardGroupId {
//     fn owners_card_groups<'a>(
//         &self,
//         game: &'a Game,
//     ) -> Result<&'a HashMap<String, CardGroup>, String> {
//         match &self.owner {
//             CardGroupOwner::Name(owner_name) => {
//                 if owner_name == COMMUNAL_CARDS {
//                     Ok(&game.communal_cards)
//                 } else {
//                     return Err(format!(
//                         "Invalid card group owner name. Right now only 'communal_cards' is supported. Given: {}",
//                         owner_name,
//                     ).into());
//                 }
//             }
//
//             CardGroupOwner::RelativePlayer {
//                 offset_from_current_player,
//             } => {
//                 let player_count = game.players().len();
//                 if let Some(player) = game.offset_from_current_player(*offset_from_current_player) {
//                     Ok(&player.hand)
//                 } else {
//                     return Err(format!(
//                         "Invalid player index. Given: {}. Player count: {}",
//                         offset_from_current_player, player_count,
//                     ));
//                 }
//             }
//         }
//     }
//
//     fn owners_card_groups_mut<'a>(
//         &self,
//         game: &'a mut Game,
//     ) -> Result<&'a mut HashMap<String, CardGroup>, String> {
//         match &self.owner {
//             CardGroupOwner::Name(owner_name) => {
//                 if owner_name == COMMUNAL_CARDS {
//                     Ok(&mut game.communal_cards)
//                 } else {
//                     return Err(format!(
//                         "Invalid card group owner name. Right now only 'communal_cards' is supported. Given: {}",
//                         owner_name,
//                     ).into());
//                 }
//             }
//
//             CardGroupOwner::RelativePlayer {
//                 offset_from_current_player,
//             } => {
//                 let player_count = game.players.len();
//                 if let Some(player) =
//                     game.offset_from_current_player_mut(*offset_from_current_player)
//                 {
//                     Ok(&mut player.hand)
//                 } else {
//                     return Err(format!(
//                         "Invalid player index. Given: {}. Player count: {}",
//                         offset_from_current_player, player_count,
//                     ));
//                 }
//             }
//         }
//     }
//
//     fn card_group<'a>(&self, game: &'a Game) -> ServiceResult<&'a CardGroup> {
//         let owners_card_groups = self.owners_card_groups(game)?;
//         let owners_card_groups_names = owners_card_groups
//             .keys()
//             .map(|k| k.to_string())
//             .collect::<Vec<String>>()
//             .join(", ");
//
//         if let Some(name) = &self.name {
//             if owners_card_groups.contains_key(name) {
//                 return Ok(owners_card_groups.get(name).unwrap());
//             } else {
//                 return Err(format!(
//                     "Player hand card group name doesn't match anything. Given: {}. Available: {}",
//                     name, owners_card_groups_names,
//                 )
//                 .into());
//             }
//         } else if let Some(first_with_cards_of) = &self.first_with_cards_of {
//             for card_group_name in first_with_cards_of {
//                 if owners_card_groups.contains_key(card_group_name) {
//                     if owners_card_groups.get(card_group_name).unwrap().cards.len() > 0 {
//                         return Ok(owners_card_groups.get(card_group_name).unwrap());
//                     }
//                 }
//             }
//
//             return Err(format!(
//                 "None of the card groups had any cards in: {}",
//                 first_with_cards_of
//                     .iter()
//                     .map(|k| k.to_string())
//                     .collect::<Vec<String>>()
//                     .join(", "),
//             ));
//         }
//
//         return Err(
//             "Invalid card group identifier. Neither name nor 'first_with_cards_of' set".into(),
//         );
//     }
//
//     fn card_group_mut<'a>(&self, game: &'a mut Game) -> ServiceResult<&'a mut CardGroup> {
//         let owners_card_groups = self.owners_card_groups_mut(game)?;
//         let owners_card_groups_names = owners_card_groups
//             .keys()
//             .map(|k| k.to_string())
//             .collect::<Vec<String>>()
//             .join(", ");
//
//         if let Some(name) = &self.name {
//             if owners_card_groups.contains_key(name) {
//                 return Ok(owners_card_groups.get_mut(name).unwrap());
//             } else {
//                 return Err(format!(
//                     "Player hand card group name doesn't match anything. Given: {}. Available: {}",
//                     name, owners_card_groups_names,
//                 )
//                 .into());
//             }
//         } else if let Some(first_with_cards_of) = &self.first_with_cards_of {
//             for card_group_name in first_with_cards_of {
//                 if owners_card_groups.contains_key(card_group_name) {
//                     if owners_card_groups.get(card_group_name).unwrap().cards.len() > 0 {
//                         return Ok(owners_card_groups.get_mut(card_group_name).unwrap());
//                     }
//                 }
//             }
//
//             return Err(format!(
//                 "None of the card groups had any cards in: {}",
//                 first_with_cards_of
//                     .iter()
//                     .map(|k| k.to_string())
//                     .collect::<Vec<String>>()
//                     .join(", "),
//             ));
//         }
//
//         return Err(
//             "Invalid card group identifier. Neither name nor 'first_with_cards_of' set".into(),
//         );
//     }
// }

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

// impl CardMove {
//     fn execute(&self, game: &mut Game) -> Result<(), String> {
//         // TODO: Prompt user to figure out what cards they want to move
//         // See if their selection meets the card conditions
//         // See if their selection is OK by the active consequences
//         // If so, move them
//         Ok(())
//     }
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardSwap {
    first_card_group: CardGroupId,
    second_card_group: CardGroupId,
}

// // Swaps cards between two card groups
// impl CardSwap {
//     fn execute(&self, game: &mut Game) -> ServiceResult<()> {
//         Ok(())
//         // let card_group_ids = vec![&self.first_card_group, &self.second_card_group];
//         //
//         // // This function reflects the length of card_groups above
//         // let other_card_group_id = |card_group_index: usize| -> &CardGroupId {
//         //     card_group_ids[(card_group_index + 1) % 2]
//         // };
//         //
//         // let mut cards_to_move_by_card_group: Vec<Vec<Card>> = vec![vec![], vec![]];
//         //
//         // for (card_group_index, card_group_id) in card_group_ids.iter().enumerate() {
//         //     loop {
//         //         println!(
//         //             "Select card to move from {} into {}. Use 0-{} or -1 to finish:",
//         //             card_group_id,
//         //             other_card_group_id(card_group_index),
//         //             &card_group_id.card_group(game)?.cards.len() - 1,
//         //         );
//         //
//         //         let selected_card_index: isize = read!();
//         //         if selected_card_index < 0 {
//         //             break;
//         //         }
//         //
//         //         if selected_card_index as usize >= card_group_id.card_group(game)?.cards.len() {
//         //             return Err(format!("Invalid card index: {}", selected_card_index));
//         //         }
//         //
//         //         cards_to_move_by_card_group[card_group_index].push(
//         //             card_group_id
//         //                 .card_group(game)?
//         //                 .cards
//         //                 .get(selected_card_index as usize)
//         //                 .unwrap()
//         //                 .clone(),
//         //         );
//         //     }
//         // }
//         //
//         // if cards_to_move_by_card_group[0].len() != cards_to_move_by_card_group[1].len() {
//         //     return Err("Card swaps must move the same number of cards in both directions".into());
//         // }
//         //
//         // for (card_group_index, (card_group_id, cards_to_move_out_of_card_group)) in card_group_ids
//         //     .iter()
//         //     .zip(cards_to_move_by_card_group.into_iter())
//         //     .enumerate()
//         // {
//         //     let card_group_cards = &mut card_group_id.card_group_mut(game)?.cards;
//         //     for card_ref in cards_to_move_out_of_card_group.iter() {
//         //         let index = card_group_cards.iter().position(|x| x == card_ref).unwrap();
//         //         card_group_cards.remove(index);
//         //     }
//         //     for card in cards_to_move_out_of_card_group.into_iter() {
//         //         other_card_group_id(card_group_index)
//         //             .card_group_mut(game)?
//         //             .cards
//         //             .push(card);
//         //     }
//         // }
//         //
//         // Ok(())
//     }
// }

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
//
// impl Operator {
//     pub fn compare(&self, lhs: usize, rhs: usize) -> bool {
//         match self {
//             Operator::Equal => lhs == rhs,
//             Operator::GreaterThan => lhs > rhs,
//             Operator::GreaterThanOrEqual => lhs >= rhs,
//             Operator::LessThan => lhs < rhs,
//             Operator::LessThanOrEqual => lhs <= rhs,
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum CardCondition {
    CardsMustBeSameRank,
    CardsMustBeHigherRankThanLastPlayedOrWild,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Condition {
    LastPlayedCardRank {
        card_group_name: CardGroupId,
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

// impl Condition {
//     fn met(&self, game: &mut Game) -> Result<bool, String> {
//         match self {
//             Condition::LastPlayedCardRank {
//                 card_group_name,
//                 equals,
//             } => {
//                 let card_group = card_group_name.card_group(game)?;
//                 if let Some(last_card_in_group) = card_group.cards.last() {
//                     Ok(last_card_in_group.rank == CardRank::from_usize(*equals))
//                 } else {
//                     Ok(false)
//                 }
//             }
//             Condition::CardGroupSize {
//                 card_group_name,
//                 operator,
//                 compare_to,
//             } => {
//                 let card_group = card_group_name.card_group(game)?;
//                 Ok(operator.compare(card_group.cards.len(), *compare_to))
//             }
//
//             Condition::TurnCount {
//                 operator,
//                 compare_to,
//             } => Ok(operator.compare(game.turn_count, *compare_to)),
//
//             Condition::CardCount {
//                 operator,
//                 compare_to,
//                 for_players,
//             } => {
//                 let players_with_card_count =
//                     game.players
//                         .iter()
//                         .fold(0, |players_with_card_count, player| {
//                             if operator.compare(
//                                 player
//                                     .hand
//                                     .values()
//                                     .map(|hand_card_group| hand_card_group.cards.len())
//                                     .sum(),
//                                 *compare_to,
//                             ) {
//                                 players_with_card_count + 1
//                             } else {
//                                 players_with_card_count
//                             }
//                         });
//
//                 match for_players {
//                     PlayerCount::AllPlayers => Ok(players_with_card_count == game.players.len()),
//                     PlayerCount::AllButOnePlayer => {
//                         Ok((game.players.len() - players_with_card_count) == 1)
//                     }
//                     PlayerCount::SomePlayers { player_count } => {
//                         Ok(players_with_card_count == *player_count)
//                     }
//                 }
//             }
//         }
//     }
// }

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

// impl Action {
//     fn available(&self, game: &Game) -> Result<bool, String> {
//         for condition in self.conditions.iter() {
//             if !condition.met(game)? {
//                 return Ok(false);
//             }
//         }
//
//         // TODO: Check if action is ruled out by any of the active consequences
//
//         return Ok(true);
//     }
//
//     pub fn execute(&self, game: &mut Game) -> ServiceResult<()> {
//         match &self.verb {
//             Verb::MoveCards(card_move) => {
//                 card_move.execute(game)?;
//
//                 Ok(())
//             }
//             Verb::SwapCards(card_swap) => {
//                 card_swap.execute(game)?;
//
//                 Ok(())
//             }
//             _ => panic!("AHHHHHHHHHHHHHHHHHHHHHHHH"),
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurnRange {
    Bounded { min: usize, max: usize },
    LowerBounded { min: usize },
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

#[derive(Clone, Debug, Deserialize)]
pub struct GameRules {
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub player_hand: Vec<CardGroupDescription>,
    pub communal_cards: Vec<CardGroupDescription>,
    pub cards: HashMap<Rank, CardValue>,
    pub game_flow: Vec<TurnType>,
}

impl GameRules {
    pub fn available_actions(&self, game: &mut Game) -> ServiceResult<Vec<&Action>> {
        Ok(Vec::new())
        // self.game_flow.iter().try_fold(
        //     Vec::new(),
        //     |mut available_actions, turn_type| -> Result<Vec<&Action>, String> {
        //         for condition in turn_type.conditions.iter() {
        //             if condition.met(game)? {
        //                 for turn_phase in turn_type.turn_phases.iter() {
        //                     for action in turn_phase.actions.iter() {
        //                         if action.available(game)? {
        //                             available_actions.push(action);
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //
        //         return Ok(available_actions);
        //     },
        // )
    }
}
