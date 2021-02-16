use std::sync::Arc;

use actix_web::http::Method;
use actix_web::{web, Error as ActixWebError, HttpRequest, HttpResponse};

use juniper::http::{playground::playground_source, GraphQLRequest};

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::graphql::{create_graphql_context, SchemaGraphQL};
use crate::user::model::LoggedInUser;

pub async fn graphql(
    req: HttpRequest,
    graphql_schema: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<GraphQLRequest>>,
    data_body: Option<web::Json<GraphQLRequest>>,
    user: LoggedInUser,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, ActixWebError> {
    // Forbid all graphql access if you aren't authenticated
    if user.0.is_none() {
        return Err(ServiceError::Unauthorized)?;
    }

    let data = match *req.method() {
        Method::GET => data_query.unwrap().into_inner(),
        _ => data_body.unwrap().into_inner(),
    };

    let db_pool = (*db_pool).clone();
    let graphql_context = create_graphql_context(user, db_pool);
    let res = data.execute(&graphql_schema, &graphql_context).await;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
