//! Main entry point for the Reading Notes API server

use actix_web::{middleware::Logger, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use reading_notes_backend::{create_app, establish_connection};
use std::env;

/// Server configuration loaded from environment variables
struct ServerConfig {
    host: String,
    port: u16,
}

impl ServerConfig {
    /// Loads server configuration from environment variables
    ///
    /// # Environment Variables
    /// - `SERVER_HOST`: Server bind address (default: 127.0.0.1)
    /// - `SERVER_PORT`: Server port (default: 8080)
    fn from_env() -> Self {
        // Ensure DATABASE_URL is set (fail fast if not)
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid port number");

        Self { host, port }
    }

    /// Returns the server bind address as a tuple
    fn bind_address(&self) -> (String, u16) {
        (self.host.clone(), self.port)
    }

    /// Returns the server URL for display
    fn server_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    initialize_environment();

    // Load configuration
    let config = ServerConfig::from_env();

    // Setup database
    let pool = establish_connection();
    log::info!("Database pool created successfully");

    // Start server
    start_server(config, pool).await
}

/// Initializes the application environment
/// - Loads .env file
/// - Configures logging
fn initialize_environment() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger with default filter
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting Reading Notes API...");
}

/// Starts the HTTP server with the given configuration
async fn start_server(
    config: ServerConfig,
    pool: reading_notes_backend::DbPool,
) -> std::io::Result<()> {
    log::info!("Starting server at {}", config.server_url());

    let bind_address = config.bind_address();

    HttpServer::new(move || create_app(pool.clone()).wrap(Logger::default()))
        .bind(bind_address)?
        .run()
        .await
}
