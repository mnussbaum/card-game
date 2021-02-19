use std::convert::From;

use juniper::{graphql_object, FieldResult};

use crate::deck::records::{Card, CardGroupLayout, CardGroupRecord, CardGroupVisibility};
use crate::errors::ServiceResult;
use crate::game::record::GameRecord;
use crate::graphql::Context;
use crate::user::model::User;

pub struct CardGroup<'a> {
    marker: std::marker::PhantomData<&'a ()>,
    record: CardGroupRecord,
}

impl<'a> CardGroup<'a> {
    pub fn find_by_game_and_user(
        context: &Context<'a>,
        game: &GameRecord,
        user: &User,
    ) -> ServiceResult<Vec<CardGroup<'a>>> {
        let connection = &context.db_pool.get()?;
        Ok(
            CardGroupRecord::find_by_game_and_user(connection, game, user)?
                .into_iter()
                .map(|record| record.into())
                .collect(),
        )
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

    fn initial_size(&self) -> i32 {
        self.record.initial_size
    }

    fn visibility(&self) -> &CardGroupVisibility {
        &self.record.visibility
    }

    fn layout(&self) -> &CardGroupLayout {
        &self.record.layout
    }

    fn cards(&self, context: &Context<'a>) -> FieldResult<Vec<Card>> {
        let connection = &context.db_pool.get()?;
        Ok(Card::belonging_to_card_group(connection, &self.record)?)
    }
}

impl<'a> From<CardGroupRecord> for CardGroup<'a> {
    fn from(record: CardGroupRecord) -> CardGroup<'a> {
        return CardGroup {
            marker: std::marker::PhantomData,
            record,
        };
    }
}
