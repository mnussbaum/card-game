use std::collections::HashMap;

use crate::card_deck::CardGroup;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub card_pools: HashMap<String, CardGroup>,
}
