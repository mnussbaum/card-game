use crate::errors::ServiceResult;
use crate::user::model::{InsertableUser, SlimUser, User, UserData};
use actix_web::web;
use diesel::prelude::*;

pub fn register(
    user_data: UserData,
    db_pool: web::Data<crate::db::Pool>,
) -> ServiceResult<SlimUser> {
    let connection = &db_pool.get()?;
    create_user(user_data, connection)
}

pub fn create_user(user_data: UserData, connection: &PgConnection) -> ServiceResult<SlimUser> {
    use crate::schema::users::dsl::users;

    let user: InsertableUser = user_data.into();
    let inserted_user: User = diesel::insert_into(users)
        .values(&user)
        .get_result(connection)?;
    Ok(inserted_user.into())
}
