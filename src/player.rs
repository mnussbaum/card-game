use std::convert::From;

use juniper::graphql_object;

use crate::deck::graphql::CardGroup;
use crate::game::graphql::GameState;
use crate::user::model::User;

pub struct Player {
    user: User,
    card_groups: Vec<CardGroup>,
}

#[graphql_object]
#[graphql(description = "A game player")]
impl Player {
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

impl From<(User, Vec<CardGroup>)> for Player {
    fn from(user_and_card_groups: (User, Vec<CardGroup>)) -> Player {
        let user = user_and_card_groups.0;
        let card_groups = user_and_card_groups.1;

        Player { user, card_groups }
    }
}
