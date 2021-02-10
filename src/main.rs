#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::*;
use rocket_contrib::databases::database;
use rocket_contrib::json::Json;

use models::*;
use schema::*;

#[database("postgres")]
struct DbConn(diesel::PgConnection);

fn main() {
    dotenv().ok();
    rocket::ignite()
        // .mount("/", routes![create_game, get_game, update_game])
        .mount("/", routes![hmm])
        .attach(DbConn::fairing())
        .launch();
}

#[get("/")]
fn hmm(db_conn: DbConn) -> Json<Vec<Player>> {
    use diesel::pg::expression::dsl::any;

    let new_player = NewPlayer { name: "steve" };
    let player: Player = diesel::insert_into(players::table)
        .values(&new_player)
        .get_result(&*db_conn)
        .expect("Error saving player");

    // let new_game = NewGame {
    //     player_turn_index: 0,
    // };
    // let game: Game = diesel::insert_into(games::table)
    //     .values(&new_game)
    //     .get_result(&*db_conn)
    //     .expect("Error saving game");
    // let players = players::table
    //     .filter(players::id.eq(any(games_players_ids)))
    //     .load::<Player>(&*db_conn)
    //     .expect("could not load players");
    let game: Game = games::table.find(1).first(&*db_conn).unwrap();
    let players = players::table
        .filter(players::id.eq(1))
        .load::<Player>(&*db_conn)
        .expect("could not load players");

    let new_game_player = NewGamePlayer {
        game_id: game.id,
        player_id: player.id,
    };
    let _: GamesPlayer = diesel::insert_into(games_players::table)
        .values(&new_game_player)
        .get_result(&*db_conn)
        .expect("Error saving game");

    let games_players_ids = GamesPlayer::belonging_to(&game).select(games_players::player_id);
    let players = players::table
        .filter(players::id.eq(any(games_players_ids)))
        .load::<Player>(&*db_conn)
        .expect("could not load players");

    Json(players)
}

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
