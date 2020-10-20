use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
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
                rank: Rank::from_usize(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Diamond,
                rank: Rank::from_usize(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Heart,
                rank: Rank::from_usize(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Spade,
                rank: Rank::from_usize(rank),
            });
        }

        return deck;
    }
}

#[derive(Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Rank {
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
    fn from_usize(rank: usize) -> Rank {
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
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
