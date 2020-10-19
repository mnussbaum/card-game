use std::collections::HashMap;

mod card_and_deck;
use card_and_deck::{Card, Deck};

fn main() {
    let players: Vec<Player> = vec![
        Player {
            name: "Alice".into(),
            hand: Default::default(),
        },
        Player {
            name: "Bob".into(),
            hand: Default::default(),
        },
    ];
    let deck: Deck = Default::default();
    let deck = deck.shuffle();
    let game_state = GameState { deck, players };
    let game_state = deal(game_state);
    println!("{:#?}", game_state);
}

fn deal(mut game_state: GameState) -> GameState {
    let player_count = game_state.players.len();

    for player_index in (0..player_count).cycle() {
        let player = &mut game_state.players[player_index];
        if let Some(card) = game_state.deck.cards.pop() {
            player
                .hand
                .cards
                .insert(player.hand.cards.len() as u32, card);
        } else {
            break;
        }
    }

    return game_state;
}

#[derive(Debug)]
struct GameState {
    deck: Deck,
    players: Vec<Player>,
}

#[derive(Debug, Default)]
struct Hand {
    cards: HashMap<u32, Card>,
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Hand,
}
