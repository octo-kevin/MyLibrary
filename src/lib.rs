//! Personal Reading Notes Management System Backend
//! 
//! This library provides RESTful APIs for managing books, reading notes,
//! categories, tags, and reading progress tracking.

// Module declarations
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod utils;

// Re-exports for convenience
pub use db::{DbPool, establish_connection};

use actix_web::{web, App};
use actix_cors::Cors;

/// Creates and configures the Actix-web application
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Configured Actix-web App instance with all routes and middleware
pub fn create_app(pool: DbPool) -> App<impl actix_web::dev::ServiceFactory<
    actix_web::dev::ServiceRequest,
    Config = (),
    Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
    InitError = (),
>> {
    let cors = configure_cors();
    
    App::new()
        // Inject database pool into app data
        .app_data(web::Data::new(pool))
        // Apply CORS middleware
        .wrap(cors)
        // Configure API routes
        .service(configure_api_routes())
}

/// Configures CORS settings for the application
/// 
/// Allows requests from common frontend development servers
/// and sets appropriate headers and methods
fn configure_cors() -> Cors {
    Cors::default()
        // Allow common frontend development servers
        .allowed_origin("http://localhost:3000")  // React default
        .allowed_origin("http://localhost:5173")  // Vite default
        // Allow standard HTTP methods
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        // Allow necessary headers
        .allowed_headers(vec!["Content-Type", "Authorization"])
        // Cache preflight requests for 1 hour
        .max_age(3600)
}

/// Configures all API routes under the /api prefix
fn configure_api_routes() -> actix_web::Scope {
    web::scope("/api")
        // Health check endpoint
        .route("/health", web::get().to(handlers::health_check))
        // TODO: Add book management routes
        // TODO: Add reading notes routes
        // TODO: Add category routes
        // TODO: Add tag routes
        // TODO: Add reading status routes
}