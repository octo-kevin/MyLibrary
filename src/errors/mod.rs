//! Error handling module for the application
//!
//! Provides a centralized error type that can be converted to HTTP responses

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

/// Main application error type
///
/// All errors in the application should be converted to this type
/// for consistent error handling and HTTP response generation
#[derive(Error, Debug)]
pub enum AppError {
    /// Generic internal server error
    #[error("Internal server error")]
    InternalError,

    /// Bad request with custom message
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Resource not found with description
    #[error("Not found: {0}")]
    NotFound(String),

    /// Validation error with details
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Database operation error
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Error response structure sent to clients
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error type/code for client handling
    #[schema(example = "VALIDATION_ERROR")]
    pub error: String,
    /// Human-readable error message
    #[schema(example = "Title is required")]
    pub message: String,
}

impl ResponseError for AppError {
    /// Converts the error into an HTTP response
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: self.error_type(),
            message: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }

    /// Maps error variants to HTTP status codes
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EnvVarError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JsonError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl AppError {
    /// Returns a string identifier for the error type
    fn error_type(&self) -> String {
        match self {
            AppError::InternalError => "INTERNAL_ERROR",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::EnvVarError(_) => "CONFIGURATION_ERROR",
            AppError::JsonError(_) => "JSON_ERROR",
        }
        .to_string()
    }
}

/// Convert actix-web JSON payload errors to our AppError
impl From<actix_web::error::JsonPayloadError> for AppError {
    fn from(err: actix_web::error::JsonPayloadError) -> Self {
        AppError::BadRequest(format!("Invalid JSON: {}", err))
    }
}

/// Convenience type alias for Results with AppError
pub type Result<T> = std::result::Result<T, AppError>;
