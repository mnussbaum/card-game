use chrono::NaiveDateTime;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};

use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

use crate::db::PooledConnection;
use crate::errors::ServiceResult;
use crate::models::NewGameUser;
use crate::schema::{games, games_users, users};
use crate::user::model::SlimUser;

#[derive(GraphQLObject, Identifiable, Queryable, Associations, Serialize)]
#[graphql(description = "A game")]
pub struct Game {
    pub id: i32,
    pub player_turn_index: i32,
    pub created_at: NaiveDateTime,
}

impl Game {
    pub fn find_by_id(
        connection: &PooledConnection,
        id: i32,
    ) -> Result<Game, diesel::result::Error> {
        games::table
            .select(games::all_columns)
            .find(id)
            .first(connection)
    }

    pub fn find_by_user_and_id(
        connection: &PooledConnection,
        user: &SlimUser,
        id: i32,
    ) -> ServiceResult<Vec<Self>> {
        Ok(users::table
            .inner_join(games_users::table.inner_join(games::table))
            .filter(users::id.eq(user.id))
            .select(games::all_columns)
            .filter(games::id.eq(id))
            .load::<Self>(connection)?)
    }

    pub fn belonging_to_user(
        connection: &PooledConnection,
        user: &SlimUser,
    ) -> ServiceResult<Vec<Self>> {
        Ok(users::table
            .inner_join(games_users::table.inner_join(games::table))
            .filter(users::id.eq(user.id))
            .select(games::all_columns)
            .load::<Self>(connection)?)
    }

    pub fn create(connection: &PooledConnection) -> ServiceResult<Self> {
        let new_game = NewGame {
            player_turn_index: 0,
        };

        Ok(diesel::insert_into(games::table)
            .values(&new_game)
            .get_result(connection)?)
    }

    pub fn join(&self, connection: &PooledConnection, user: &SlimUser) -> ServiceResult<&Self> {
        diesel::insert_into(games_users::table)
            .values(NewGameUser {
                game_id: self.id,
                user_id: user.id,
            })
            .execute(connection)?;

        Ok(self)
    }
}

#[derive(GraphQLInputObject, Insertable, Deserialize)]
#[table_name = "games"]
#[graphql(description = "New game")]
pub struct NewGame {
    pub player_turn_index: i32,
}
