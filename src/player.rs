use std::convert::From;

use juniper::{graphql_object, FieldResult};

use crate::deck::graphql::CardGroup;
use crate::game::record::GameRecord;
use crate::graphql::Context;
use crate::user::model::User;

pub struct Player<'a> {
    game_record: &'a GameRecord,
    user: User,
}

#[graphql_object(context = Context<'a>)]
#[graphql(description = "A game player")]
impl<'a> Player<'a> {
    fn id(&self) -> i32 {
        self.user.id
    }

    fn email(&self) -> &str {
        &self.user.email
    }

    fn card_groups(&self, context: &Context<'a>) -> FieldResult<Vec<CardGroup>> {
        Ok(CardGroup::find_by_game_record_and_user(
            context,
            self.game_record,
            &self.user,
        )?)
    }
}

impl<'a> From<(&'a GameRecord, User)> for Player<'a> {
    fn from(game_record_and_user: (&'a GameRecord, User)) -> Player<'a> {
        Player {
            game_record: game_record_and_user.0,
            user: game_record_and_user.1,
        }
    }
}
