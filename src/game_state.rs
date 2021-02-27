use std::collections::HashMap;

use serde::Serialize;

use crate::card_deck::{CardGroup, Deck};
use crate::game_rules::GameRules;
use crate::player::Player;

#[derive(Clone, Debug, Serialize)]
pub struct GameState {
    pub communal_cards: HashMap<String, CardGroup>,
    deck: Deck,
    game_rules: GameRules,
    player_turn_index: usize,
    pub players: Vec<Player>,
    pub turn_count: usize,
}

impl GameState {
    pub fn new(game_rules: GameRules, mut players: Vec<Player>) -> Self {
        for mut player in &mut players {
            player.hand = game_rules.player_hand.clone();
        }

        // TODO: Build deck from game rules
        let deck: Deck = Default::default();
        let deck = deck.shuffle();

        return GameState {
            communal_cards: game_rules.communal_cards.clone(),
            deck,
            game_rules,
            player_turn_index: 0,
            players,
            turn_count: 0,
        };
    }

    pub fn offset_from_current_player_mut(&mut self, offset: usize) -> Option<&mut Player> {
        let player_index = (self.player_turn_index + offset) % self.players.len();
        return self.players.get_mut(player_index);
    }

    pub fn offset_from_current_player(&self, offset: usize) -> Option<&Player> {
        let player_index = (self.player_turn_index + offset) % self.players.len();
        return self.players.get(player_index);
    }

    fn maybe_deal_card_to_card_group(
        deck: &mut Deck,
        card_group: &mut CardGroup,
    ) -> (Option<bool>, bool) {
        let mut card_group_has_an_initial_deal_count = false;

        if let Some(at_or_over_initial_deal_size) = card_group.at_or_over_initial_deal_size() {
            if at_or_over_initial_deal_size {
                return (Some(true), false);
            } else {
                card_group_has_an_initial_deal_count = true;
            }
        }

        let mut deck_empty = false;
        if let Some(card) = deck.cards.pop() {
            card_group.cards.push(card);
        } else {
            deck_empty = true;
        }

        if card_group_has_an_initial_deal_count {
            return (Some(false), deck_empty);
        } else {
            return (None, deck_empty);
        }
    }

    pub fn deal(&mut self) {
        let player_count = self.players.len();

        for (player_hand_name, _) in self.game_rules.player_hand.iter() {
            let mut hand_at_initial_deal_count_for_all_players = false;

            for player_index in (0..player_count).cycle() {
                if player_index == 0 && hand_at_initial_deal_count_for_all_players {
                    break;
                } else {
                    hand_at_initial_deal_count_for_all_players = true
                }

                let player = self
                    .players
                    .get_mut(player_index)
                    .expect("Error getting a player by index");
                let player_hand = player.hand.get_mut(player_hand_name).expect(&format!(
                    "Player {} is missing hand {}",
                    player.name, player_hand_name
                ));

                let (maybe_card_group_full, deck_empty) =
                    GameState::maybe_deal_card_to_card_group(&mut self.deck, player_hand);

                // If there aren't enough cards to finish dealing I abruptly
                // return here. Is this behavior correct? Should it be configurable?
                if deck_empty {
                    return;
                }

                if let Some(card_group_full) = maybe_card_group_full {
                    if card_group_full {
                        continue;
                    } else {
                        hand_at_initial_deal_count_for_all_players = false;
                    }
                }
            }
        }

        for (_, communal_card_group) in self.communal_cards.iter_mut() {
            let (maybe_card_group_full, deck_empty) =
                GameState::maybe_deal_card_to_card_group(&mut self.deck, communal_card_group);

            // If there aren't enough cards to finish dealing I abruptly
            // return here. Is this behavior correct? Should it be configurable?
            if deck_empty {
                return;
            }

            if let Some(card_group_full) = maybe_card_group_full {
                if card_group_full {
                    continue;
                }
            }
        }
    }

    pub fn player_on_turn(&mut self) -> &mut Player {
        return &mut self.players[self.player_turn_index];
    }

    pub fn advance_player_turn(&mut self) {
        self.player_turn_index = (self.player_turn_index + 1) % self.players.len();

        if self.player_turn_index == 0 {
            self.turn_count = self.turn_count + 1
        }
    }

    pub fn play_game(&mut self) -> Result<(), String> {
        loop {
            println!("");
            println!("Communal cards: {:?}", self.communal_cards);
            let player = self.player_on_turn();
            println!("Your cards: {:?}", player.hand);

            // This clone satisfies the borrowck and allows me to mutably pass self to
            // Action.excute
            let game_rules = self.game_rules.clone();
            let available_actions = game_rules.available_actions(self)?;
            // if let Some(selected_action) = user_input::select_action(available_actions) {
            //     selected_action.execute(self)?;
            // }

            // TODO:
            //
            //  Need to implement actions and conditions
            //  then to implement game flow into actions, eg advancing turn phase and next turn
            //
            // How do I know what cards to eval for consequences?
            // What was "just played"
            // After user plays card re-eval consequences to handle if eg the player can play again

            // play_turn(&mut game_state);

            // let player = self.player_on_turn();
            // if player.hand.cards.len() == 0 {
            //     println!("{} wins!", player.name);
            //     return;
            // }

            self.advance_player_turn();
        }
    }
}
