use std::convert::From;

use juniper::{graphql_object, FieldResult};

use crate::deck::models::CardGroup;
use crate::graphql::Context;
use crate::models::{GameUser, UserAndGameUser};
use crate::user::model::User;

pub struct Player<'a> {
    marker: std::marker::PhantomData<&'a ()>,
    user: User,
    game_user: GameUser,
}

#[graphql_object(context = Context<'a>)]
#[graphql(description = "A game player")]
impl<'a> Player<'a> {
    fn id(&self) -> i32 {
        self.game_user.id
    }

    fn email(&self) -> &str {
        &self.user.email
    }

    fn card_groups(&self, context: &Context<'a>) -> FieldResult<Vec<CardGroup>> {
        let connection = &context.db_pool.get()?;
        Ok(CardGroup::belonging_to_user(connection, &self.user)?)
    }
}

impl<'a> From<UserAndGameUser> for Player<'a> {
    fn from(user_and_game_user: UserAndGameUser) -> Player<'a> {
        return Player {
            marker: std::marker::PhantomData,
            user: user_and_game_user.0,
            game_user: user_and_game_user.1,
        };
    }
}
