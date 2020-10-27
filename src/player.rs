use crate::card_deck::Card;

#[derive(Debug, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
}
