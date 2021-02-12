use crate::errors::{ServiceError, ServiceResult};
use crate::user::model::{SlimUser, User};
use crate::user::util::verify;
use actix_web::web;
use diesel::prelude::*;

pub fn login(
    user_email: &str,
    user_password: &str,
    db_pool: web::Data<crate::db::Pool>,
) -> ServiceResult<SlimUser> {
    use crate::schema::users::dsl::{email, users};

    let connection = &db_pool.get()?;
    let user = users
        .filter(email.eq(user_email))
        .first::<User>(connection)
        .map_err(|_| ServiceError::Unauthorized)?;

    if verify(&user, &user_password) {
        Ok(user.into())
    } else {
        Err(ServiceError::Unauthorized)
    }
}
