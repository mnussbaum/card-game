use std::sync::Arc;

use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};

use crate::db::DbPool;
use crate::models::{Game, NewGame};

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<DbPool>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

// TODO: START HERE: Next do a full CRUD for games
// Then port more game fields and models into DB and graphql resources
// Figure out how best to serialize game rules
// Then migrate existing logic into new models
// Then use juniper (aka graphql)/actix/diesel examples to serve games over websockets (aka
// graphql subscriptions)
// Then use something like Apollo on the front end to receive game state
// Turn action prompts into a field on game state that can be displayed to the user

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query for games")]
    fn games(context: &Context, id: Option<i32>, player_id: Option<i32>) -> FieldResult<Vec<Game>> {
        let connection = &context.db_pool.get()?;

        let games = if let Some(id) = id {
            Game::find_by_id(connection, id)?
        } else if let Some(player_id) = player_id {
            Game::belongs_to_player_id(connection, player_id)?
        } else {
            return Err("Query requires either a game ID or a PLAYER_ID")?;
        };

        Ok(games)
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    #[graphql(description = "Add player to game")]
    fn update_game(context: &Context, data: NewGame) -> FieldResult<Game> {
        // let connection = &context.db_pool.get()?;
        //
        // let game: Game = diesel::insert_into(games::table)
        //     .values(&data)
        //     .get_result(connection)
        //     .expect("Error saving game");
        //
        // Ok(game)
        panic!("boom")
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_graphql_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn create_graphql_context(db_pool: Arc<DbPool>) -> Context {
    Context { db_pool }
}
