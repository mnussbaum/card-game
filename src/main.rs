use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

fn main() {
    let mut players: Vec<Player> = vec![
        Player {
            name: "Alice".into(),
            hand: Default::default(),
        },
        Player {
            name: "Bob".into(),
            hand: Default::default(),
        },
    ];
    let mut deck: Deck = Default::default();
    let deck = deck.shuffle();
    let mut game_state = GameState { deck, players };
    let game_state = deal(game_state);
    println!("{:#?}", game_state);
}

fn deal(mut game_state: GameState) -> GameState {
    for player in game_state.players.iter_mut() {
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

#[derive(Debug)]
struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(mut self) -> Self {
        self.cards.shuffle(&mut thread_rng());
        return self;
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut deck = Deck { cards: vec![] };
        for rank in 1..13 {
            deck.cards.push(Card {
                suit: Suit::Club,
                rank: Rank::from_u32(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Diamond,
                rank: Rank::from_u32(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Heart,
                rank: Rank::from_u32(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Spade,
                rank: Rank::from_u32(rank),
            });
        }

        return deck;
    }
}

#[derive(Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug)]
enum Rank {
    LowAce = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    HighAce = 14,
}

impl Rank {
    fn from_u32(rank: u32) -> Rank {
        match rank {
            1 => Rank::LowAce,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::HighAce,
            _ => panic!("Unknown rank: {}", rank),
        }
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
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
