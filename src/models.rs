use chrono::NaiveDateTime;

use diesel::{Insertable, Queryable};

use crate::game::model::Game;
use crate::schema::games_users;
use crate::user::model::User;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Game)]
#[belongs_to(User)]
#[table_name = "games_users"]
pub struct GameUser {
    pub id: i32,
    pub game_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "games_users"]
pub struct NewGameUser {
    pub game_id: i32,
    pub user_id: i32,
}
