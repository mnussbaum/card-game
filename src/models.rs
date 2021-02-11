use super::schema::{games, games_players, players};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, Serialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "players"]
pub struct NewPlayer<'a> {
    pub name: &'a str,
}

#[derive(GraphQLObject, Identifiable, Queryable, Associations, Serialize)]
#[graphql(description = "Game")]
pub struct Game {
    pub id: i32,
    pub player_turn_index: i32,
    // Then keep adding DB tables and models
    // Then migrate existing logic into new models
    // Then use juniper (aka graphql)/actix/diesel examples to serve games over websockets (aka
    // graphql subscriptions)
    // Then use something like Apollo on the front end to receive game state
    // Turn action prompts into a field on game state that can be displayed to the user
}

impl Game {
    pub fn find_by_id(
        connection: &super::DbPooledConnection,
        id: i32,
    ) -> Result<Vec<Game>, diesel::result::Error> {
        games::table
            .filter(games::id.eq(id))
            .load::<Game>(connection)
    }

    pub fn belongs_to_player_id(
        connection: &super::DbPooledConnection,
        player_id: i32,
    ) -> Result<Vec<Game>, diesel::result::Error> {
        players::table
            .inner_join(games_players::table.inner_join(games::table))
            .filter(players::id.eq(player_id))
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
#[belongs_to(Player)]
#[table_name = "games_players"]
pub struct GamePlayer {
    pub id: i32,
    pub game_id: i32,
    pub player_id: i32,
}

#[derive(Insertable)]
#[table_name = "games_players"]
pub struct NewGamePlayer {
    pub game_id: i32,
    pub player_id: i32,
}
