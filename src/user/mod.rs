mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::user::handler::{login, login_view, logout, me, register, register_view};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(
                web::resource("/register")
                    .route(web::post().to(register))
                    .route(web::get().to(register_view)),
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(login))
                    .route(web::get().to(login_view)),
            )
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(web::resource("/me").route(web::get().to(me))),
    );
}
