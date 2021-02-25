use crate::graphql::Context;
use juniper::{graphql_object, FieldResult};

use crate::deck::records::{Card, CardGroupLayout, CardGroupRecord, CardGroupVisibility};

pub struct CardGroup<'a> {
    marker: std::marker::PhantomData<&'a ()>,
    record: CardGroupRecord,
    cards: Vec<Card>,
}

impl<'a> CardGroup<'a> {
    pub fn new(record: CardGroupRecord, cards: Vec<Card>) -> CardGroup<'a> {
        CardGroup {
            cards,
            marker: std::marker::PhantomData,
            record,
        }
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
