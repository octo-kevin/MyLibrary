//! Database connection pool management
//!
//! Provides connection pooling for PostgreSQL using r2d2

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::time::Duration;

/// Type alias for the database connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Database pool configuration
struct PoolConfig {
    /// Maximum number of connections in the pool
    max_size: u32,
    /// Minimum number of idle connections to maintain
    min_idle: u32,
    /// Connection timeout in seconds
    timeout_seconds: u64,
}

impl PoolConfig {
    /// Loads pool configuration from environment variables
    ///
    /// # Environment Variables
    /// - `DATABASE_POOL_MAX_SIZE`: Maximum connections (default: 10)
    /// - `DATABASE_POOL_MIN_IDLE`: Minimum idle connections (default: 2)
    /// - `DATABASE_POOL_TIMEOUT_SECONDS`: Connection timeout (default: 30)
    fn from_env() -> Self {
        Self {
            max_size: parse_env_var("DATABASE_POOL_MAX_SIZE", 10),
            min_idle: parse_env_var("DATABASE_POOL_MIN_IDLE", 2),
            timeout_seconds: parse_env_var("DATABASE_POOL_TIMEOUT_SECONDS", 30),
        }
    }
}

/// Establishes a connection pool to the PostgreSQL database
///
/// # Panics
/// - If DATABASE_URL is not set
/// - If the pool cannot be created
///
/// # Returns
/// A configured r2d2 connection pool
pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = PoolConfig::from_env();

    create_pool(&database_url, config)
}

/// Creates a connection pool with the given configuration
fn create_pool(database_url: &str, config: PoolConfig) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(config.max_size)
        .min_idle(Some(config.min_idle))
        .connection_timeout(Duration::from_secs(config.timeout_seconds))
        .build(manager)
        .expect("Failed to create database pool")
}

/// Parses an environment variable as a numeric type with a default value
fn parse_env_var<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(default)
}
