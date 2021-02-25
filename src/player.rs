use std::convert::From;

use juniper::graphql_object;

use crate::deck::graphql::CardGroup;
use crate::graphql::Context;
use crate::user::model::User;

pub struct Player<'a> {
    user: User,
    card_groups: Vec<CardGroup<'a>>,
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

    fn card_groups(&self) -> &Vec<CardGroup> {
        &self.card_groups
    }
}

impl<'a> From<(User, Vec<CardGroup<'a>>)> for Player<'a> {
    fn from(user_and_card_groups: (User, Vec<CardGroup<'a>>)) -> Player<'a> {
        let user = user_and_card_groups.0;
        let card_groups = user_and_card_groups.1;

        Player { user, card_groups }
    }
}
