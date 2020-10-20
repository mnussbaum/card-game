use std::collections::HashMap;

use text_io::read;

mod card_and_deck;
use card_and_deck::{Card, Deck};

fn main() {
    let mut players: HashMap<usize, Player> = HashMap::new();
    players.insert(
        0,
        Player {
            name: "Alice".into(),
            hand: Default::default(),
        },
    );
    players.insert(
        1,
        Player {
            name: "Bob".into(),
            hand: Default::default(),
        },
    );
    let deck: Deck = Default::default();
    let deck = deck.shuffle();
    let communal_cards: Vec<Card> = vec![];
    let game_state = GameState {
        deck,
        communal_cards,
        players,
    };
    let game_state = deal(game_state);
    play_game(game_state)
}

fn play_game(mut game_state: GameState) {
    let player_count = game_state.players.len();

    for player_index in (0..player_count).cycle() {
        if let Some(mut player) = game_state.players.remove(&player_index) {
            println!("Your cards: {:#?}", player.hand.cards);
            println!("Last played card: {:#?}", game_state.communal_cards.last());

            play_turn(&mut game_state, &mut player);

            if player.hand.cards.len() == 0 {
                println!("{} wins!", player.name);
                game_state.players.insert(player_index, player);
                return;
            }

            game_state.players.insert(player_index, player);
        } else {
            panic!("Tried to play a non-existent player!");
        }
    }
}

fn play_turn(game_state: &mut GameState, player: &mut Player) {
    println!("What card do you want to play:");
    let card_to_play_index: usize = read!();
    if let Some(card_to_play) = player.hand.cards.remove(&card_to_play_index) {
        if valid_card_to_play(&game_state, &card_to_play) {
            // play card
            game_state.communal_cards.push(card_to_play);
        } else {
            println!("Sorry, you can't play that card!");
            player.hand.cards.insert(card_to_play_index, card_to_play);
            return play_turn(game_state, player);
        }
    } else {
        println!("Please pick a valid card index");
        return play_turn(game_state, player);
    }
}

fn valid_card_to_play(game_state: &GameState, card_to_play: &Card) -> bool {
    if let Some(last_played_card) = game_state.communal_cards.last() {
        if card_to_play.rank >= last_played_card.rank {
            true
        } else {
            false
        }
    } else {
        true
    }
}

fn deal(mut game_state: GameState) -> GameState {
    let player_count = game_state.players.len();

    for player_index in (0..player_count).cycle() {
        if let Some(mut player) = game_state.players.remove(&player_index) {
            if let Some(card) = game_state.deck.cards.pop() {
                player
                    .hand
                    .cards
                    .insert(player.hand.cards.len() as usize, card);
                game_state.players.insert(player_index, player);
            } else {
                game_state.players.insert(player_index, player);
                break;
            }
        } else {
            panic!("Tried to play a non-existent player!");
        }
    }

    return game_state;
}

#[derive(Debug)]
struct GameState {
    communal_cards: Vec<Card>,
    deck: Deck,
    players: HashMap<usize, Player>,
}

#[derive(Debug, Default)]
struct Hand {
    cards: HashMap<usize, Card>,
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Hand,
}
