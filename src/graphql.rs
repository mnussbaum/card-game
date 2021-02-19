use std::sync::Arc;

use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::game::{graphql::Game, record::GameRecord};
use crate::user::model::{LoggedInUser, SlimUser};

#[derive(Clone)]
pub struct Context<'a> {
    // The phantom data allows the use of a lifetime param. The lifetime param
    // allows authenticated_user to return a pointer to context data with a
    // context-scoped lifetime
    marker: std::marker::PhantomData<&'a ()>,

    pub db_pool: Arc<Pool>,
    pub user: LoggedInUser,
}

impl<'a> juniper::Context for Context<'a> {}

impl<'a> Context<'a> {
    pub fn authenticated_user(&'a self) -> Result<&'a SlimUser, ServiceError> {
        if let Some(logged_in_user) = &self.user.0 {
            return Ok(logged_in_user);
        }

        Err(ServiceError::Unauthorized)
    }
}

pub struct QueryRoot<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

// TODO: START HERE:
// * Migrate game state into models and API
// * When users request a game also give them their available actions
// * Only let users see cards they have perms for
// * Move user CRUD into graphql and out of REST
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

#[graphql_object(context = Context<'a>)]
impl<'a> QueryRoot<'a> {
    #[graphql(description = "Query for games")]
    fn games(context: &Context<'a>, id: Option<i32>) -> FieldResult<Vec<Game>> {
        let connection = &context.db_pool.get()?;
        let user = context.authenticated_user()?;

        let game_records = if let Some(id) = id {
            GameRecord::find_by_user_and_id(connection, user, id)?
        } else {
            GameRecord::belonging_to_user(connection, user)?
        };

        Ok(game_records.into_iter().map(|g| g.into()).collect())
    }
}

pub struct MutationRoot<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

#[graphql_object(context = Context<'a>)]
impl<'a> MutationRoot<'a> {
    #[graphql(description = "Create a new game")]
    fn create_game(context: &Context<'a>) -> FieldResult<Game> {
        Ok(GameRecord::create(&context.db_pool.get()?)?.into())
    }

    #[graphql(description = "Add a player to game")]
    fn join_game(context: &Context<'a>, game_id: i32) -> FieldResult<Game> {
        let user = context.authenticated_user()?;
        let connection = &context.db_pool.get()?;
        let game = GameRecord::find_by_id(connection, game_id)?;

        game.join(connection, user)?;

        Ok(game.into())
    }
}

pub type SchemaGraphQL = RootNode<
    'static,
    QueryRoot<'static>,
    MutationRoot<'static>,
    EmptySubscription<Context<'static>>,
>;

pub fn create_graphql_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(
        QueryRoot {
            marker: std::marker::PhantomData,
        },
        MutationRoot {
            marker: std::marker::PhantomData,
        },
        EmptySubscription::new(),
    )
}

pub fn create_graphql_context<'a>(user: LoggedInUser, db_pool: Arc<Pool>) -> Context<'a> {
    Context {
        db_pool,
        marker: std::marker::PhantomData,
        user,
    }
}
