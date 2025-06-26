use diesel::r2d2::{self, ConnectionManager};
use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use reading_notes_backend::DbPool;
use std::env;
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Test database wrapper that automatically creates a unique database for each test
/// and cleans it up when dropped
pub struct TestDb {
    /// Connection pool for the test database
    pub pool: DbPool,
    /// Unique database name (format: my_library_test_{uuid_v7})
    pub db_name: String,
    /// Base PostgreSQL connection URL (without database name)
    base_url: String,
}

impl Drop for TestDb {
    fn drop(&mut self) {
        // Note: The pool will be dropped automatically when TestDb is dropped
        // We just need to ensure we can connect to postgres to drop the database

        // Connect to postgres database to drop the test database
        let postgres_url = format!("{}/postgres", self.base_url);
        if let Ok(mut conn) = PgConnection::establish(&postgres_url) {
            let drop_query = format!("DROP DATABASE IF EXISTS {}", self.db_name);
            diesel::sql_query(&drop_query).execute(&mut conn).ok();
        }
    }
}

/// Creates a new test database with a unique name for isolation
///
/// # Returns
/// A `TestDb` instance that will automatically clean up the database when dropped
///
/// # Panics
/// - If DATABASE_URL environment variable is not set
/// - If database creation fails
/// - If migrations fail to run
pub fn setup_test_db() -> TestDb {
    // Load test environment configuration
    dotenv::from_filename(".env.test").ok();

    let base_database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

    // Parse the database URL to extract base URL and database name
    let (base_url, _original_db_name) = parse_database_url(&base_database_url);

    // Generate a unique test database name using UUID v7 (time-ordered)
    let unique_db_name = generate_test_db_name();
    let test_database_url = format!("{}/{}", base_url, unique_db_name);

    // Create the test database
    create_test_database(&base_url, &unique_db_name);

    // Setup connection pool
    let pool = create_connection_pool(&test_database_url);

    // Run database migrations
    run_migrations(&pool);

    TestDb {
        pool,
        db_name: unique_db_name,
        base_url: base_url.to_string(),
    }
}

/// Parses a database URL into base URL and database name components
fn parse_database_url(url: &str) -> (&str, &str) {
    let url_parts: Vec<&str> = url.rsplitn(2, '/').collect();
    if url_parts.len() != 2 {
        panic!("Invalid database URL format. Expected: postgres://user:pass@host:port/dbname");
    }
    (url_parts[1], url_parts[0])
}

/// Generates a unique test database name using UUID v7
fn generate_test_db_name() -> String {
    format!("my_library_test_{}", Uuid::now_v7().simple())
}

/// Creates a connection pool for the given database URL
fn create_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create test database pool")
}

/// Runs pending migrations on the database
fn run_migrations(pool: &DbPool) {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

/// Creates a new PostgreSQL database for testing
///
/// # Arguments
/// * `base_url` - PostgreSQL connection URL without database name
/// * `db_name` - Name for the new database (will be validated)
///
/// # Panics
/// - If connection to PostgreSQL fails
/// - If database name contains invalid characters
/// - If database creation fails
fn create_test_database(base_url: &str, db_name: &str) {
    validate_database_name(db_name);

    let postgres_url = format!("{}/postgres", base_url);
    let mut conn = PgConnection::establish(&postgres_url)
        .unwrap_or_else(|e| panic!("Could not connect to PostgreSQL: {}", e));

    // Create the database (name is pre-validated, safe to use in query)
    let create_query = format!("CREATE DATABASE {}", db_name);
    diesel::sql_query(&create_query)
        .execute(&mut conn)
        .unwrap_or_else(|e| panic!("Failed to create test database '{}': {}", db_name, e));
}

/// Validates that a database name is safe to use in SQL queries
///
/// # Arguments
/// * `db_name` - Database name to validate
///
/// # Panics
/// If the database name contains invalid characters or is invalid length
fn validate_database_name(db_name: &str) {
    const MAX_POSTGRES_NAME_LENGTH: usize = 63;
    const ALLOWED_CHARS: &str = "alphanumeric characters, underscores, and hyphens";

    // Check for empty name
    if db_name.is_empty() {
        panic!("Database name cannot be empty");
    }

    // Check length constraint (PostgreSQL limit)
    if db_name.len() > MAX_POSTGRES_NAME_LENGTH {
        panic!(
            "Database name '{}' is too long ({} chars). Maximum length is {} characters.",
            db_name,
            db_name.len(),
            MAX_POSTGRES_NAME_LENGTH
        );
    }

    // Check for valid characters (prevent SQL injection)
    let has_invalid_chars = !db_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-');

    if has_invalid_chars {
        panic!(
            "Database name '{}' contains invalid characters. Only {} are allowed.",
            db_name, ALLOWED_CHARS
        );
    }
}
