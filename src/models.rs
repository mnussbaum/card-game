use chrono::NaiveDateTime;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};

use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

use crate::db::PooledConnection;
use crate::schema::{games, games_users, users};
use crate::user::model::{SlimUser, User};

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
    ) -> Result<Vec<Game>, diesel::result::Error> {
        users::table
            .inner_join(games_users::table.inner_join(games::table))
            .filter(users::id.eq(user.id))
            .select(games::all_columns)
            .filter(games::id.eq(id))
            .load::<Game>(connection)
    }

    pub fn belonging_to_user(
        connection: &PooledConnection,
        user: &SlimUser,
    ) -> Result<Vec<Game>, diesel::result::Error> {
        users::table
            .inner_join(games_users::table.inner_join(games::table))
            .filter(users::id.eq(user.id))
            .select(games::all_columns)
            .load::<Game>(connection)
    }
}

#[derive(GraphQLInputObject, Insertable, Deserialize)]
#[table_name = "games"]
#[graphql(description = "New game")]
pub struct NewGame {
    pub player_turn_index: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Game)]
#[belongs_to(User)]
#[table_name = "games_users"]
pub struct GameUser {
    pub id: i32,
    pub game_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "games_users"]
pub struct NewGameUser {
    pub game_id: i32,
    pub user_id: i32,
}
