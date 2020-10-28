use std::collections::HashMap;

use crate::card_deck::CardGroup;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: HashMap<String, CardGroup>,
}
