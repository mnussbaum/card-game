use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::user::model::{LoggedInUser, SlimUser, UserData};
use crate::user::service as user;
use serde::Deserialize;

impl FromRequest for LoggedInUser {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();

        let slim_user = if let Some(identity) = identity {
            match serde_json::from_str::<SlimUser>(&identity) {
                Err(e) => return futures::future::err(e.into()),
                Ok(slim_user) => Ok(Some(slim_user)),
            }
        } else {
            Ok(None)
        };

        futures::future::ready(slim_user.map(LoggedInUser))
    }
}

pub async fn register(
    user_data: web::Form<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user::register(user_data.into_inner(), pool)
        .map(|slim_user| HttpResponse::Ok().json(&slim_user))
}

pub(super) async fn register_view() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("static/register.html")))
}

#[derive(Debug, Deserialize)]
pub(super) struct LoginQuery {
    pub email: String,
    pub password: String,
}

pub(super) async fn login(
    auth_data: web::Form<LoginQuery>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user::login(&auth_data.email, &auth_data.password, pool).and_then(|slim_user| {
        let slim_user_json =
            serde_json::to_string(&slim_user).map_err(|_| ServiceError::InternalServerError)?;
        id.remember(slim_user_json);

        Ok(HttpResponse::Ok().json(slim_user))
    })
}

pub(super) async fn login_view() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("static/login.html")))
}

pub fn me(logged_in_user: LoggedInUser) -> HttpResponse {
    match logged_in_user.0 {
        None => HttpResponse::Unauthorized().json(ServiceError::Unauthorized),
        Some(slim_user) => HttpResponse::Ok().json(slim_user),
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();

    HttpResponse::Ok().finish()
}
