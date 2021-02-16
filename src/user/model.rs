use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::*;
use crate::user::util::{make_hash, make_salt};

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    pub user_uuid: Uuid,
    #[graphql(skip)]
    pub hash: Vec<u8>,
    #[graphql(skip)]
    pub salt: String,
    pub email: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub user_uuid: Uuid,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

// SlimUser represents the user info stored in a cookie
#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub user_uuid: Uuid,
    pub email: String,
}

#[derive(Clone, Default)]
pub struct LoggedInUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedInUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedInUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            name,
            email,
            password,
            ..
        } = user_data;

        let salt = make_salt();
        let hash = make_hash(&password, &salt).to_vec();
        Self {
            user_uuid: Uuid::new_v4(),
            email,
            hash,
            created_at: chrono::Local::now().naive_local(),
            salt,
            name,
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            user_uuid, email, ..
        } = user;

        Self { user_uuid, email }
    }
}
