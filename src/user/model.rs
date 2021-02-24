use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::users;
use crate::user::util::{make_hash, make_salt};

#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    Identifiable,
    Serialize,
    Deserialize,
    Queryable,
    juniper::GraphQLObject,
)]
pub struct User {
    pub id: i32,
    #[graphql(skip)]
    pub hash: Vec<u8>,
    #[graphql(skip)]
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
}

// SlimUser represents the user info stored in a cookie
#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub email: String,
    pub id: i32,
}

#[derive(Clone, Debug, Default)]
pub struct LoggedInUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedInUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedInUser(Some(slim_user))
    }
}

impl From<CreateUserInput> for InsertableUser {
    fn from(user_data: CreateUserInput) -> Self {
        let CreateUserInput {
            email, password, ..
        } = user_data;

        let salt = make_salt();
        let hash = make_hash(&password, &salt).to_vec();
        Self {
            email,
            hash,
            created_at: chrono::Local::now().naive_local(),
            salt,
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User { id, email, .. } = user;

        Self { id, email }
    }
}
