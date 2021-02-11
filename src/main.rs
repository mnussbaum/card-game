#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod db;
pub mod graphql_schema;
pub mod models;
pub mod schema;

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::http::Method;
use actix_web::{
    middleware, web, App, Error as ActixWebError, HttpRequest, HttpResponse, HttpServer,
};
use juniper::http::{playground::playground_source, GraphQLRequest};

use dotenv::dotenv;

use db::{create_db_pool, DbPool};
use graphql_schema::{create_graphql_context, create_graphql_schema, SchemaGraphQL};

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

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<GraphQLRequest>>,
    data_body: Option<web::Json<GraphQLRequest>>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ActixWebError> {
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
    let ctx = create_graphql_context(db_pool);
    let res = data.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
