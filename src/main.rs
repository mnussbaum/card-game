#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use std::io;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use std::env;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;

use models::*;
use schema::*;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let graphql_schema = std::sync::Arc::new(create_graphql_schema());

    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    let db_pool = Pool::builder().build(manager).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(graphql_schema.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("*")
            //         .allowed_methods(vec!["GET", "POST"]),
            // )
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            }))
    })
    .bind("localhost:8000")?
    .run()
    .await
}

pub async fn woo() -> HttpResponse {
    let html = "hi";
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// #[get("/")]
// fn hmm(db_conn: DbConn) -> Json<Vec<Player>> {
//     use diesel::pg::expression::dsl::any;
//
//     let new_player = NewPlayer { name: "steve" };
//     let player: Player = diesel::insert_into(players::table)
//         .values(&new_player)
//         .get_result(&*db_conn)
//         .expect("Error saving player");
//
//     // let new_game = NewGame {
//     //     player_turn_index: 0,
//     // };
//     // let game: Game = diesel::insert_into(games::table)
//     //     .values(&new_game)
//     //     .get_result(&*db_conn)
//     //     .expect("Error saving game");
//     // let players = players::table
//     //     .filter(players::id.eq(any(games_players_ids)))
//     //     .load::<Player>(&*db_conn)
//     //     .expect("could not load players");
//     let game: Game = games::table.find(1).first(&*db_conn).unwrap();
//     let players = players::table
//         .filter(players::id.eq(1))
//         .load::<Player>(&*db_conn)
//         .expect("could not load players");
//
//     let new_game_player = NewGamePlayer {
//         game_id: game.id,
//         player_id: player.id,
//     };
//     let _: GamesPlayer = diesel::insert_into(games_players::table)
//         .values(&new_game_player)
//         .get_result(&*db_conn)
//         .expect("Error saving game");
//
//     let games_players_ids = GamesPlayer::belonging_to(&game).select(games_players::player_id);
//     let players = players::table
//         .filter(players::id.eq(any(games_players_ids)))
//         .load::<Player>(&*db_conn)
//         .expect("could not load players");
//
//     Json(players)
// }

// #[post("/games/create")]
// fn create_game(db_conn: DbConn) -> Json<card_table::GameState> {
//     let new_game = card_table::new_game().unwrap();
//
//     Json(games[&uid].clone())
// }
//
// #[get("/games/<uid>")]
// fn get_game(db_conn: DbConn) -> Json<card_table::GameState> {
//     let games = games.read().unwrap();
//     Json(games[&uid].clone())
// }
//
// // #[put("/games/<uid>/update", format = "application/json", data = "<game>")]
// #[put("/games/<uid>/update")]
// fn update_game(db_conn: DbConn) {}

use std::sync::Arc;

use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<DbPool>,
}

impl juniper::Context for Context {}

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub struct QueryRoot;

// TODO: START HERE: Have game by ID working.
// Next do games by player
// Then do a full CRUD for games
// Then port more game fields and models into DB and graphql resources
// Figure out how best to serialize game rules
// Separate graphql code from main and DB code from main

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query a game status")]
    fn game(context: &Context, id: i32) -> FieldResult<Game> {
        let connection = &context.db_pool.get()?;

        let mut games = games::table
            .filter(games::id.eq(id))
            .load::<Game>(connection)
            .expect("could not load game");
        Ok(games.pop().unwrap())
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    #[graphql(description = "Add player to game")]
    fn update_game(context: &Context, data: NewGame) -> FieldResult<Game> {
        let connection = &context.db_pool.get()?;

        let game: Game = diesel::insert_into(games::table)
            .values(&data)
            .get_result(connection)
            .expect("Error saving game");

        Ok(game)
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_graphql_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn create_context(db_pool: Arc<DbPool>) -> Context {
    Context { db_pool }
}

use actix_web::http::header::HeaderMap;
use actix_web::http::Method;
use actix_web::{Error, HttpRequest};
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::serde::ser::Error as SerdeError;

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<GraphQLRequest>>,
    data_body: Option<web::Json<GraphQLRequest>>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let data = match *req.method() {
        Method::GET => data_query.unwrap().into_inner(),
        _ => data_body.unwrap().into_inner(),
    };

    // let introspection queries through
    // if data.operation_name() != Some("IntrospectionQuery") {
    //     // validate key for all other requests
    //     if let Err(e) = validate_key(&headers, key.get_ref()) {
    //         let err = GraphQLErrors::new(e);
    //
    //         return Ok(HttpResponse::Ok().json(&err));
    //     }
    // }

    let db_pool = (*db_pool).clone();
    let ctx = create_context(db_pool);
    let res = data.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}
