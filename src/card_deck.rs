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
                rank: CardRank::from_usize(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Diamond,
                rank: CardRank::from_usize(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Heart,
                rank: CardRank::from_usize(rank),
            });
            deck.cards.push(Card {
                suit: Suit::Spade,
                rank: CardRank::from_usize(rank),
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

pub enum CardValue {
    Wild,
    Numeric(usize),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum CardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardRank {
    pub fn from_usize(rank: usize) -> CardRank {
        match rank {
            2 => CardRank::Two,
            3 => CardRank::Three,
            4 => CardRank::Four,
            5 => CardRank::Five,
            6 => CardRank::Six,
            7 => CardRank::Seven,
            8 => CardRank::Eight,
            9 => CardRank::Nine,
            10 => CardRank::Ten,
            11 => CardRank::Jack,
            12 => CardRank::Queen,
            13 => CardRank::King,
            14 => CardRank::Ace,
            _ => panic!("Unknown rank: {}", rank),
        }
    }

    pub fn unicode_code_point(&self) -> u32 {
        match self {
            CardRank::Two => 0x2,
            CardRank::Three => 0x3,
            CardRank::Four => 0x4,
            CardRank::Five => 0x5,
            CardRank::Six => 0x6,
            CardRank::Seven => 0x7,
            CardRank::Eight => 0x8,
            CardRank::Nine => 0x9,
            CardRank::Ten => 0xA,
            CardRank::Jack => 0xB,
            CardRank::Queen => 0xD,
            CardRank::King => 0xE,
            CardRank::Ace => 0x1,
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: CardRank,
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

#[derive(Debug)]
pub enum CardGroupVisibility {
    FaceDown,
    FaceUp,
    TopFaceUpRestFaceDown,
    VisibleToOwner,
}

#[derive(Debug)]
pub struct CardGroup {
    pub cards: Vec<Card>,
    pub max_card_limit: Option<usize>,
    pub visibility: CardGroupVisibility,
}
