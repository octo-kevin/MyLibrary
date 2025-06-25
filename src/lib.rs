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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation configuration
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::books::create_book,
        handlers::books::get_book,
        handlers::books::list_books,
        handlers::books::update_book,
        handlers::books::delete_book,
        handlers::notes::create_note,
        handlers::notes::get_note,
        handlers::notes::list_notes,
        handlers::notes::get_book_notes,
        handlers::notes::update_note,
        handlers::notes::update_note_tags,
        handlers::notes::delete_note,
        handlers::tags::create_tag,
        handlers::tags::get_tag,
        handlers::tags::list_tags,
        handlers::tags::get_popular_tags,
        handlers::tags::update_tag,
        handlers::tags::delete_tag,
    ),
    components(
        schemas(
            models::book::CreateBookRequest,
            models::book::BookResponse,
            models::book::BookListResponse,
            models::book::UpdateBook,
            models::note::CreateNoteRequest,
            models::note::NoteResponse,
            models::note::NoteListResponse,
            models::note::UpdateReadingNote,
            models::note::NoteType,
            models::tag::CreateTagRequest,
            models::tag::TagResponse,
            models::tag::TagListResponse,
            models::tag::PopularTagResponse,
            models::tag::UpdateTag,
            errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Books", description = "Book management operations"),
        (name = "Notes", description = "Reading note management operations"),
        (name = "Tags", description = "Tag management operations")
    ),
    info(
        title = "Personal Reading Notes API",
        description = "API for managing personal reading notes, books, and reading progress",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server")
    )
)]
pub struct ApiDoc;

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
        // Add Swagger UI
        .service(
            SwaggerUi::new("/docs/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
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
        // Book management routes
        .service(configure_book_routes())
        // Note management routes
        .service(configure_note_routes())
        // Tag management routes
        .service(configure_tag_routes())
        // TODO: Add category routes
        // TODO: Add reading status routes
}

/// Configures book management routes
fn configure_book_routes() -> actix_web::Scope {
    web::scope("/books")
        .route("", web::post().to(handlers::books::create_book))
        .route("", web::get().to(handlers::books::list_books))
        .route("/{id}", web::get().to(handlers::books::get_book))
        .route("/{id}", web::put().to(handlers::books::update_book))
        .route("/{id}", web::delete().to(handlers::books::delete_book))
        .route("/{book_id}/notes", web::get().to(handlers::notes::get_book_notes))
}

/// Configures note management routes
fn configure_note_routes() -> actix_web::Scope {
    web::scope("/notes")
        .route("", web::post().to(handlers::notes::create_note))
        .route("", web::get().to(handlers::notes::list_notes))
        .route("/{id}", web::get().to(handlers::notes::get_note))
        .route("/{id}", web::put().to(handlers::notes::update_note))
        .route("/{id}", web::delete().to(handlers::notes::delete_note))
        .route("/{id}/tags", web::put().to(handlers::notes::update_note_tags))
}

/// Configures tag management routes
fn configure_tag_routes() -> actix_web::Scope {
    web::scope("/tags")
        .route("", web::post().to(handlers::tags::create_tag))
        .route("", web::get().to(handlers::tags::list_tags))
        .route("/popular", web::get().to(handlers::tags::get_popular_tags))
        .route("/{id}", web::get().to(handlers::tags::get_tag))
        .route("/{id}", web::put().to(handlers::tags::update_tag))
        .route("/{id}", web::delete().to(handlers::tags::delete_tag))
}