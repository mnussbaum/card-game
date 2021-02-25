use std::collections::HashMap;
use std::convert::From;

use juniper::graphql_object;

use crate::deck::graphql::CardGroup;
use crate::graphql::Context;
use crate::user::model::User;

pub struct Player<'a> {
    user: User,
    card_groups: HashMap<String, CardGroup<'a>>,
}

impl<'a> Player<'a> {
    pub fn get_card_group_mut(&mut self, card_group_name: &str) -> Option<&mut CardGroup<'a>> {
        self.card_groups.get_mut(card_group_name)
    }

    pub fn id(&self) -> i32 {
        self.user.id
    }
}

#[graphql_object(context = Context<'a>)]
#[graphql(description = "A game player")]
impl<'a> Player<'a> {
    fn id(&self) -> i32 {
        self.id()
    }

    fn email(&self) -> &str {
        &self.user.email
    }

    fn card_groups(&self) -> Vec<&CardGroup> {
        self.card_groups.values().collect()
    }
}

impl<'a> From<(User, HashMap<String, CardGroup<'a>>)> for Player<'a> {
    fn from(user_and_card_groups: (User, HashMap<String, CardGroup<'a>>)) -> Player<'a> {
        let user = user_and_card_groups.0;
        let card_groups = user_and_card_groups.1;

        Player { user, card_groups }
    }
}
