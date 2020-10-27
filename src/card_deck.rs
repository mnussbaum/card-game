use rand::seq::SliceRandom;
use rand::thread_rng;
use std::char;
use std::fmt;

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
        for rank in 2..15 {
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

impl Suit {
    pub fn unicode_code_point(&self) -> u32 {
        match self {
            Suit::Spade => 0xA0,
            Suit::Heart => 0xB0,
            Suit::Diamond => 0xC0,
            Suit::Club => 0xD0,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Rank {
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
    Ace = 14,
}

impl Rank {
    pub fn from_usize(rank: usize) -> Rank {
        match rank {
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
            14 => Rank::Ace,
            _ => panic!("Unknown rank: {}", rank),
        }
    }

    pub fn unicode_code_point(&self) -> u32 {
        match self {
            Rank::Two => 0x2,
            Rank::Three => 0x3,
            Rank::Four => 0x4,
            Rank::Five => 0x5,
            Rank::Six => 0x6,
            Rank::Seven => 0x7,
            Rank::Eight => 0x8,
            Rank::Nine => 0x9,
            Rank::Ten => 0xA,
            Rank::Jack => 0xB,
            Rank::Queen => 0xD,
            Rank::King => 0xE,
            Rank::Ace => 0x1,
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

const PLAYING_CARD_UNICODE_CODE_POINT_LOWER_BOUND: u32 = 0x1F000;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            char::from_u32(
                PLAYING_CARD_UNICODE_CODE_POINT_LOWER_BOUND
                    + self.suit.unicode_code_point()
                    + self.rank.unicode_code_point()
            )
            .expect(&format!(
                "Invalid card to unicode conversion for: {:#?} {:#?}",
                self.rank, self.suit
            ))
            .to_string(),
        )
    }
}
