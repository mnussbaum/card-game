use std::collections::HashMap;
use std::fs;

use text_io::read;

mod card_deck;
use card_deck::{Card, CardGroup, CardRank, Deck};

mod player;
use player::Player;

mod game_rules;
use game_rules::GameRules;

fn main() {
    let yams =
        fs::read_to_string("poo_head_rules.yaml").expect("Something went wrong reading the file");
    let game_rules: GameRules = serde_yaml::from_str(&yams).unwrap();
    // TODO: Get players from user. Use min and max player count from game rules
    let mut players: Vec<Player> = vec![
        Player {
            name: "Alice".into(),
            hand: HashMap::new(),
        },
        Player {
            name: "Bob".into(),
            hand: HashMap::new(),
        },
    ];

    for mut player in &mut players {
        player.hand = game_rules.player_hand.clone();
    }

    // TODO: Build deck from game rules
    let deck: Deck = Default::default();
    let deck = deck.shuffle();

    let communal_cards = game_rules.communal_cards.clone();
    let player_turn_index = 0;
    let game_state = GameState {
        deck,
        communal_cards,
        player_turn_index,
        players,
    };

    let game_state = deal(&game_rules, game_state);
    println!("{:?}", game_state.players[0].hand);
    println!("{:?}", game_state.communal_cards);

    play_game(&game_rules, game_state)
}

fn play_game(game_rules: &GameRules, mut game_state: GameState) {
    loop {
        // Get actions player can take from rules
        // Get consequences from last played card
        // Apply consequences
        //
        // println!("Last played card: {:#?}", game_state.communal_cards.last());
        // let player = game_state.player_on_turn();
        // println!("Your cards: {:#?}", player.hand.cards);
        //
        // play_turn(&mut game_state);
        //
        // let player = game_state.player_on_turn();
        // if player.hand.cards.len() == 0 {
        //     println!("{} wins!", player.name);
        //     return;
        // }
        //
        // game_state.advance_player_turn();
    }
}

// fn play_turn(game_state: &mut GameState) {
//     if game_state.communal_cards.len() > 0 {
//         println!("Pick it up (y/N):");
//         let pick_it_up_answer: String = read!();
//         // let player = game_state.player_on_turn();
//         if pick_it_up_answer == "y" {
//             // pick_it_up(game_state, player);
//             pick_it_up(game_state);
//             return;
//         }
//     }
//
//     println!("What card do you want to play:");
//     let card_to_play_index: usize = read!();
//
//     let player = game_state.player_on_turn();
//     if card_to_play_index >= player.hand.cards.len() {
//         println!("Sorry, card index out of range!");
//         return play_turn(game_state);
//     }
//
//     let card_to_play = player.hand.cards.remove(card_to_play_index);
//     if valid_card_to_play(game_state, &card_to_play) {
//         if card_to_play.rank == Rank::from_usize(3) {
//             // TODO: Determine next player
//             // TODO: Pick it up on next player
//             // TODO: Skip next player
//             return;
//         }
//
//         if card_to_play.rank == Rank::from_usize(10) {
//             game_state.communal_cards.drain(0..);
//             play_turn(game_state);
//             return;
//         }
//
//         // TODO: Need to encode wild card into rank comparisons
//
//         if let Some(last_played_card) = game_state.communal_cards.last() {
//             if last_played_card.rank == card_to_play.rank {
//                 game_state.communal_cards.push(card_to_play);
//                 println!("PUSHING");
//                 // TODO: Skip next player
//                 return;
//             }
//         }
//
//         game_state.communal_cards.push(card_to_play);
//     } else {
//         println!("Sorry, you can't play that card!");
//         let player = game_state.player_on_turn();
//         player.hand.cards.insert(card_to_play_index, card_to_play);
//         return play_turn(game_state);
//     }
// }
//
// fn pick_it_up(game_state: &mut GameState) {
//     // fn pick_it_up(game_state: &mut GameState, player: &mut Player) {
//     return;
// }
//
// fn valid_card_to_play(game_state: &GameState, card_to_play: &Card) -> bool {
//     if let Some(last_played_card) = game_state.communal_cards.last() {
//         if card_to_play.rank >= last_played_card.rank {
//             true
//         } else {
//             false
//         }
//     } else {
//         true
//     }
// }

#[derive(Debug)]
struct GameState {
    communal_cards: HashMap<String, CardGroup>,
    deck: Deck,
    player_turn_index: usize,
    players: Vec<Player>,
}

impl GameState {
    pub fn player_on_turn(&mut self) -> &mut Player {
        return &mut self.players[self.player_turn_index];
    }

    pub fn advance_player_turn(&mut self) {
        self.player_turn_index = (self.player_turn_index + 1) % self.players.len();
    }
}

fn deal(game_rules: &GameRules, mut game_state: GameState) -> GameState {
    let player_count = game_state.players.len();

    for (player_hand_name, player_hand_rules) in game_rules.player_hand.iter() {
        let mut hand_at_initial_deal_count_for_all_players = false;

        for player_index in (0..player_count).cycle() {
            if player_index == 0 && hand_at_initial_deal_count_for_all_players {
                break;
            } else {
                hand_at_initial_deal_count_for_all_players = true
            }

            let player = game_state
                .players
                .get_mut(player_index)
                .expect("Error getting a player by index");
            let player_hand = player.hand.get_mut(player_hand_name).expect(&format!(
                "Player {} is missing hand {}",
                player.name, player_hand_name
            ));

            // TODO: Push this into a method on the card group
            if let Some(initial_deal_count) = player_hand_rules.initial_deal_count {
                if player_hand.cards.len() >= initial_deal_count {
                    continue;
                } else {
                    hand_at_initial_deal_count_for_all_players = false;
                }
            }

            if let Some(card) = game_state.deck.cards.pop() {
                player_hand
                    .cards
                    .insert(player_hand.cards.len() as usize, card);
            } else {
                // If there aren't enough cards to finish dealing I abruptly
                // return here. Is this behavior correct? Should it be configurable?
                return game_state;
            }
        }
    }

    for (_, communal_card_group) in game_state.communal_cards.iter_mut() {
        if let Some(initial_deal_count) = communal_card_group.initial_deal_count {
            if communal_card_group.cards.len() >= initial_deal_count {
                continue;
            }
        }

        if let Some(card) = game_state.deck.cards.pop() {
            communal_card_group
                .cards
                .insert(communal_card_group.cards.len() as usize, card);
        } else {
            // If there aren't enough cards to finish dealing I abruptly
            // return here. Is this behavior correct? Should it be configurable?
            return game_state;
        }
    }

    return game_state;
}
