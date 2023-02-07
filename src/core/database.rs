use anyhow::Result;

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

/// shorten type for pooled connections
pub type DB = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
/// shorten type for postgres database pooled connections
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

/// generic function to generate a connection pool from a connection URL. This will use r2d2 pooling system
/// to enable shared database connection pool across the multiple threads of the running API.
pub fn get_connection_pool(conn_url: String) -> Result<PostgresPool> {
    let migr = ConnectionManager::<PgConnection>::new(conn_url);

    let pool = r2d2::Pool::builder().build(migr)?;

    Ok(pool)
}

/// A struct representing managed state of the database connection pool.
#[derive(Clone)]
pub struct DbPoolState {
    /// the actual database connection pool
    pub db_pool: PostgresPool,
}
