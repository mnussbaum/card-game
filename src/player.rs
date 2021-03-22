use std::collections::HashMap;
use std::convert::From;

use diesel::prelude::*;
use juniper::graphql_object;

use crate::db::PooledConnection;
use crate::deck::graphql::CardGroup;
use crate::deck::records::{CardGroupDescription, CardGroupRecord, NewCardGroupRecord};
use crate::errors::ServiceResult;
use crate::graphql::Context;
use crate::schema::card_groups;
use crate::user::model::User;

pub struct Player<'a> {
    user: User,
    card_groups: HashMap<String, CardGroup<'a>>,
    game_id: i32,
}

impl<'a> Player<'a> {
    pub fn new(
        game_id: i32,
        user: User,
        card_groups: HashMap<String, CardGroup<'a>>,
    ) -> Player<'a> {
        Player {
            game_id,
            user,
            card_groups,
        }
    }

    pub fn id(&self) -> i32 {
        self.user.id
    }

    pub fn card_groups_by_name(&self) -> &HashMap<String, CardGroup> {
        &self.card_groups
    }

    pub fn create_card_group_from_description(
        &mut self,
        description: &CardGroupDescription,
        connection: &PooledConnection,
    ) -> ServiceResult<&mut CardGroup<'a>> {
        let mut new_card_group_record =
            NewCardGroupRecord::new_from_description(description, self.game_id);

        new_card_group_record.user_id = Some(self.id());
        let card_group_record: CardGroupRecord = diesel::insert_into(card_groups::table)
            .values(new_card_group_record)
            .returning(card_groups::all_columns)
            .get_result::<CardGroupRecord>(connection)?;

        self.card_groups.insert(
            card_group_record.name.clone(),
            CardGroup::new(card_group_record, vec![]),
        );

        Ok(self
            .card_groups
            .get_mut(&description.name)
            .expect("Just inserted card group is missing on player state"))
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
