use std::convert::From;

use juniper::{graphql_object, FieldResult};

use crate::deck::graphql::CardGroup;
use crate::game::record::GameRecord;
use crate::graphql::Context;
use crate::models::{GameAndGameUserAndUser, GameUser};
use crate::user::model::User;

pub struct Player<'a> {
    user: User,
    game_user: GameUser,
    game: &'a GameRecord,
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
        Ok(CardGroup::find_by_game_and_user(
            context, self.game, &self.user,
        )?)
    }
}

impl<'a> From<GameAndGameUserAndUser<'a>> for Player<'a> {
    fn from(game_and_game_user_and_game: GameAndGameUserAndUser<'a>) -> Player<'a> {
        Player {
            game: game_and_game_user_and_game.game,
            game_user: game_and_game_user_and_game.game_user,
            user: game_and_game_user_and_game.user,
        }
    }
}
