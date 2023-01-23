use std::error::Error;

use diesel::{r2d2::ConnectionManager, PgConnection};
use failure::Fail;
use r2d2::Pool;

pub type DB = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(conn_url: String) -> Result<PostgresPool, Box<dyn Error>> {
    let migr = ConnectionManager::<PgConnection>::new(conn_url);

    let pool = r2d2::Pool::builder().build(migr)?;

    Ok(pool)
}

#[derive(Clone)]
pub struct DbPoolState {
    pub db_pool: PostgresPool,
}

#[derive(Debug, Fail)]
pub enum DBRequestResultError {
    #[fail(display = "Item(s) not found in database.")]
    NotFound,
}
