use std::collections::HashMap;
use std::fs;

mod player;
use player::Player;

mod card_deck;

mod game_rules;
use game_rules::GameRules;

mod game_state;
use game_state::GameState;

fn main() {
    let yams =
        fs::read_to_string("poo_head_rules.yaml").expect("Something went wrong reading the file");
    let game_rules: GameRules = serde_yaml::from_str(&yams).unwrap();

    // TODO: Get players from user. Use min and max player count from game rules
    let players: Vec<Player> = vec![
        Player {
            name: "Alice".into(),
            hand: HashMap::new(),
        },
        Player {
            name: "Bob".into(),
            hand: HashMap::new(),
        },
    ];

    let mut game_state = GameState::new(game_rules, players);
    game_state.deal();

    game_state.play_game();
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
