#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod db;
pub mod graphql_schema;
pub mod models;
pub mod route_handlers;
pub mod schema;

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use dotenv::dotenv;

use db::create_db_pool;
use graphql_schema::create_graphql_schema;
use route_handlers::{graphql, playground};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let graphql_schema = Arc::new(create_graphql_schema());
    let db_pool = create_db_pool().expect("failed to create DB pool");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(graphql_schema.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"]),
            )
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
