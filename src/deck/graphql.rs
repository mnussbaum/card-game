use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

use crate::db::PooledConnection;
use crate::deck::records::{
    Card, CardGroupLayout, CardGroupRecord, CardGroupVisibility, Deck, NewCardGroupCard,
};
use crate::errors::ServiceResult;
use crate::graphql::Context;
use crate::schema::card_groups_cards;

pub struct CardGroup<'a> {
    marker: std::marker::PhantomData<&'a ()>,
    record: CardGroupRecord,
    pub cards: Vec<Card>,
}

impl<'a> CardGroup<'a> {
    pub fn new(record: CardGroupRecord, cards: Vec<Card>) -> CardGroup<'a> {
        CardGroup {
            cards,
            marker: std::marker::PhantomData,
            record,
        }
    }

    pub fn at_or_over_initial_size(&self) -> Option<bool> {
        if let Some(initial_size) = self.record.initial_size {
            if self.cards.len() as i32 >= initial_size {
                return Some(true);
            } else {
                return Some(false);
            }
        }

        return None;
    }

    // Deals a card from the deck if the card group isn't full and if the deck
    // isn't empty
    pub fn deal_card_from_deck_if_not_full(
        &mut self,
        deck: &mut Deck,
        connection: &PooledConnection,
    ) -> ServiceResult<bool> {
        if let Some(at_or_over_initial_size) = self.at_or_over_initial_size() {
            if at_or_over_initial_size {
                return Ok(true);
            }
        }

        if let Some(card) = deck.cards.pop() {
            self.add_card(card, connection)?;
        }

        if let Some(at_or_over_initial_size) = self.at_or_over_initial_size() {
            if at_or_over_initial_size {
                return Ok(true);
            }
        }

        return Ok(false);
    }

    pub fn add_card(&mut self, card: Card, connection: &PooledConnection) -> ServiceResult<()> {
        diesel::insert_into(card_groups_cards::table)
            .values(&NewCardGroupCard {
                card_id: card.id,
                card_group_id: self.record.id,
            })
            .execute(connection)?;

        self.cards.push(card);

        Ok(())
    }
}

#[graphql_object(context = Context<'a>)]
#[graphql(description = "A group of cards")]
impl<'a> CardGroup<'a> {
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

    fn cards(&self, context: &Context<'a>) -> FieldResult<Vec<Card>> {
        let connection = &context.db_pool.get()?;
        let mut cards = self.cards.clone();

        match self.record.visibility {
            CardGroupVisibility::FaceUp => Ok(cards),
            CardGroupVisibility::FaceDown => {
                Ok(Card::repeat_covered_card(connection, cards.len())?)
            }

            CardGroupVisibility::TopFaceUpRestFaceDown => {
                if cards.len() <= 0 {
                    return Ok(cards);
                }

                let top_card = cards.remove(0);
                let mut covered_cards = Card::repeat_covered_card(connection, cards.len())?;
                covered_cards.insert(0, top_card);

                Ok(covered_cards)
            }

            CardGroupVisibility::VisibleToOwner => {
                let authenticated_user = context.authenticated_user()?;
                if let Some(card_group_owner_user_id) = self.record.user_id {
                    if authenticated_user.id == card_group_owner_user_id {
                        return Ok(cards);
                    }
                }

                Ok(Card::repeat_covered_card(connection, cards.len())?)
            }
        }
    }
}
