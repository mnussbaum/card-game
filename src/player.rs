use std::collections::HashMap;

use serde::Serialize;

use crate::card_deck::CardGroup;

#[derive(Clone, Debug, Serialize)]
pub struct Player {
    pub name: String,
    pub hand: HashMap<String, CardGroup>,
}
