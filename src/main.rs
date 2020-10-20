use text_io::read;

mod card_and_deck;
use card_and_deck::{Card, Deck, Rank};

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
        // TODO: Playing out of turn like for completions
        // TODO: Move current turn holder into game state
        let mut player = game_state.players.remove(player_index);
        println!("Your cards: {:#?}", player.hand.cards);
        println!("Last played card: {:#?}", game_state.communal_cards.last());

        play_turn(&mut game_state, &mut player);

        if player.hand.cards.len() == 0 {
            println!("{} wins!", player.name);
            game_state.players.insert(player_index, player);
            return;
        }

        game_state.players.insert(player_index, player);
    }
}

fn play_turn(game_state: &mut GameState, player: &mut Player) {
    if game_state.communal_cards.len() > 0 {
        println!("Pick it up (y/N):");
        let pick_it_up_answer: String = read!();
        if pick_it_up_answer == "y" {
            pick_it_up(game_state, player);
            return;
        }
    }

    println!("What card do you want to play:");
    let card_to_play_index: usize = read!();

    if card_to_play_index >= player.hand.cards.len() {
        println!("Sorry, card index out of range!");
        return play_turn(game_state, player);
    }

    let card_to_play = player.hand.cards.remove(card_to_play_index);
    if valid_card_to_play(&game_state, &card_to_play) {
        if card_to_play.rank == Rank::from_usize(3) {
            // TODO: Determine next player
            // TODO: Pick it up on next player
            // TODO: Skip next player
            return;
        }

        if card_to_play.rank == Rank::from_usize(10) {
            game_state.communal_cards.drain(0..);
            play_turn(game_state, player);
            return;
        }

        // TODO: Need to encode wild card into rank comparisons

        if let Some(last_played_card) = game_state.communal_cards.last() {
            if last_played_card.rank == card_to_play.rank {
                game_state.communal_cards.push(card_to_play);
                // TODO: Skip next player
                return;
            }
        }

        game_state.communal_cards.push(card_to_play);
    } else {
        println!("Sorry, you can't play that card!");
        player.hand.cards.insert(card_to_play_index, card_to_play);
        return play_turn(game_state, player);
    }
}

fn pick_it_up(game_state: &mut GameState, player: &mut Player) {
    player
        .hand
        .cards
        .extend(game_state.communal_cards.drain(0..));
    return;
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
        let mut player = game_state.players.remove(player_index);
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
    }

    return game_state;
}

#[derive(Debug)]
struct GameState {
    communal_cards: Vec<Card>,
    deck: Deck,
    players: Vec<Player>,
}

#[derive(Debug, Default)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Hand,
}
