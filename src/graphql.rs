use std::sync::Arc;

use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};
use log::error;

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::models::{Game, NewGame};
use crate::user::model::LoggedInUser;

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<Pool>,
    pub user: LoggedInUser,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

// TODO: START HERE:
// * Create new game
// * Only allow users to view games they're in
// * When users request a game also give them their available actions
// * Only let users see cards they have perms for
// * Consolidate error handling
// * Add CSRF protection
// * Do I need to use blocking indicators around DB queries?
//
// * Figure out how to specify necessary user input in serialized actions
// * Prompt user for input client side
// * Send a mutation request in to take an action - include game ID, serialized action with inputs
//   filled
// * Ensure user actually has action available to them, and that filled inputs are valid
// * Execute action with inputs
//
// * Port more game fields and models into DB and graphql resources
// * Migrate existing logic into new models
// * Use websockets
// * Use something like Apollo on the front end to receive game state

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query for games")]
    fn games(context: &Context, id: Option<i32>) -> FieldResult<Vec<Game>> {
        let connection = &context.db_pool.get()?;

        if let Some(logged_in_user) = &context.user.0 {
            if let Some(id) = id {
                Ok(Game::find_by_user_and_id(connection, logged_in_user, id)?)
            } else {
                Ok(Game::belongs_to_user(connection, logged_in_user)?)
            }
        } else {
            error!("Reached a game query without a logged in user");
            return Err(ServiceError::InternalServerError)?;
        }
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

pub fn create_graphql_context(user: LoggedInUser, db_pool: Arc<Pool>) -> Context {
    Context { user, db_pool }
}
