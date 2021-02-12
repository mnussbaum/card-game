use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{
    ConnectionManager, Pool as R2D2Pool, PoolError, PooledConnection as R2D2PooledConnection,
};

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection = R2D2PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Result<Pool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );

    Pool::builder().build(manager)
}
