use std::convert::From;

use juniper::{graphql_object, FieldResult};

use crate::game::record::GameRecord;
use crate::graphql::Context;
use crate::player::Player;

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

    fn players(&self, context: &Context<'a>) -> FieldResult<Vec<Player>> {
        let connection = &context.db_pool.get()?;
        Ok(
            GameRecord::game_and_game_users_and_users_by_game(connection, &self.record)?
                .into_iter()
                .map(|user_and_game_user| user_and_game_user.into())
                .collect(),
        )
    }

    fn player_turn_index(&self) -> i32 {
        self.record.player_turn_index
    }
}

impl<'a> From<GameRecord> for Game<'a> {
    fn from(record: GameRecord) -> Game<'a> {
        return Game {
            marker: std::marker::PhantomData,
            record,
        };
    }
}
