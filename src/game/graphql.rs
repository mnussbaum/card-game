use std::convert::From;

use juniper::{graphql_object, FieldResult};
use serde::Serialize;

use crate::game::record::GameRecord;
use crate::graphql::Context;
use crate::user::model::User;

#[derive(Serialize)]
pub struct Game<'a> {
    marker: std::marker::PhantomData<&'a ()>,
    record: GameRecord,
}

#[graphql_object(context = Context<'a>)]
#[graphql(description = "A game")]
impl<'a> Game<'a> {
    fn id(&self) -> i32 {
        self.record.id
    }

    fn players(&self, context: &Context<'a>) -> FieldResult<Vec<User>> {
        let connection = &context.db_pool.get()?;
        Ok(GameRecord::users_by_game_id(connection, self.record.id)?)
    }

    fn player_turn_index(&self) -> i32 {
        self.record.player_turn_index
    }
}

impl<'a> From<GameRecord> for Game<'a> {
    fn from(record: GameRecord) -> Game<'a> {
        let marker = std::marker::PhantomData;
        return Game { marker, record };
    }
}
