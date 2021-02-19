use chrono::NaiveDateTime;

use diesel::{Insertable, Queryable};

use crate::game::record::GameRecord;
use crate::schema::games_users;
use crate::user::model::User;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(GameRecord, foreign_key = "game_id")]
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

#[derive(Queryable)]
pub struct GameAndGameUserAndUser<'a> {
    pub game: &'a GameRecord,
    pub game_user: GameUser,
    pub user: User,
}

impl<'a> From<(&'a GameRecord, GameUser, User)> for GameAndGameUserAndUser<'a> {
    fn from(
        game_and_game_user_and_game: (&'a GameRecord, GameUser, User),
    ) -> GameAndGameUserAndUser<'a> {
        GameAndGameUserAndUser {
            game: game_and_game_user_and_game.0,
            game_user: game_and_game_user_and_game.1,
            user: game_and_game_user_and_game.2,
        }
    }
}
