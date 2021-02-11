use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );

    Pool::builder().build(manager)
}