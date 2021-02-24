use std::convert::From;

use juniper::graphql_object;

use crate::deck::records::{Card, CardGroupLayout, CardGroupRecord, CardGroupVisibility};

pub struct CardGroup {
    record: CardGroupRecord,
    cards: Vec<Card>,
}

#[graphql_object]
#[graphql(description = "A group of cards")]
impl CardGroup {
    fn id(&self) -> i32 {
        self.record.id
    }

    fn name(&self) -> &str {
        &self.record.name
    }

    fn initial_size(&self) -> Option<i32> {
        self.record.initial_size
    }

    fn layout(&self) -> &CardGroupLayout {
        &self.record.layout
    }

    fn cards(&self) -> &Vec<Card> {
        match self.record.visibility {
            CardGroupVisibility::FaceUp => &self.cards,
            _ => &self.cards,
            // CardGroupVisibility::FaceDown => {
            //     Ok(Card::repeat_covered_card(connection, self.cards.len())?)
            // }
            //
            // CardGroupVisibility::TopFaceUpRestFaceDown => {
            //     if cards.len() <= 0 {
            //         return Ok(cards);
            //     }
            //
            //     let top_card = cards.remove(0);
            //     let mut covered_cards = Card::repeat_covered_card(connection, cards.len())?;
            //     covered_cards.insert(0, top_card);
            //
            //     Ok(covered_cards)
            // }
            //
            // CardGroupVisibility::VisibleToOwner => {
            //     let authenticated_user = context.authenticated_user()?;
            //     if let Some(card_group_owner_user_id) = self.record.user_id {
            //         if authenticated_user.id == card_group_owner_user_id {
            //             return Ok(cards);
            //         }
            //     }
            //
            //     Ok(Card::repeat_covered_card(connection, cards.len())?)
            // }
        }
    }
}

impl From<(CardGroupRecord, Vec<Card>)> for CardGroup {
    fn from(record_and_cards: (CardGroupRecord, Vec<Card>)) -> CardGroup {
        return CardGroup {
            record: record_and_cards.0,
            cards: record_and_cards.1,
        };
    }
}
