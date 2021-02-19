use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};

use serde::{Deserialize, Serialize};

use crate::db::PooledConnection;
use crate::errors::ServiceResult;
use crate::models::{NewGameUser, UserAndGameUser};
use crate::schema::{games, games_users, users};
use crate::user::model::SlimUser;

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[table_name = "games"]
pub struct GameRecord {
    pub id: i32,
    pub player_turn_index: i32,
    pub created_at: NaiveDateTime,
}

impl GameRecord {
    pub fn user_and_game_users_by_game_id(
        connection: &PooledConnection,
        id: i32,
    ) -> ServiceResult<Vec<UserAndGameUser>> {
        Ok(users::table
            .inner_join(games_users::table.inner_join(games::table))
            .filter(games::id.eq(id))
            .select((users::all_columns, games_users::all_columns))
            .get_results(connection)?)
    }

    pub fn find_by_id(connection: &PooledConnection, id: i32) -> ServiceResult<GameRecord> {
        Ok(games::table
            .select(games::all_columns)
            .find(id)
            .first(connection)?)
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