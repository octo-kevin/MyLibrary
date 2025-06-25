//! Tag management HTTP handlers
//! 
//! Provides RESTful API endpoints for tag CRUD operations

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use utoipa::IntoParams;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::tag::{Tag, CreateTagRequest, UpdateTag, TagListResponse};

/// Query parameters for tag listing
#[derive(Debug, Deserialize, IntoParams)]
pub struct TagListQuery {
    /// Page number (1-based, default: 1)
    #[param(example = 1)]
    pub page: Option<u32>,
    /// Items per page (default: 20, max: 100)
    #[param(example = 20)]
    pub per_page: Option<u32>,
}

/// Path parameters for tag operations
#[derive(Debug, Deserialize, IntoParams)]
pub struct TagPath {
    /// Tag ID
    #[param(example = 1)]
    pub id: i64,
}

/// Query parameters for popular tags
#[derive(Debug, Deserialize, IntoParams)]
pub struct PopularTagsQuery {
    /// Number of popular tags to return (default: 10, max: 50)
    #[param(example = 10)]
    pub limit: Option<i64>,
}

/// Creates a new tag
#[utoipa::path(
    post,
    path = "/api/tags",
    request_body = CreateTagRequest,
    responses(
        (status = 201, description = "Tag created successfully", body = TagResponse),
        (status = 400, description = "Tag already exists", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Tags"
)]
pub async fn create_tag(
    pool: web::Data<DbPool>,
    tag_data: web::Json<CreateTagRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate required fields
    if tag_data.name.trim().is_empty() {
        return Err(AppError::ValidationError("Tag name is required".to_string()));
    }

    let tag = Tag::create(&mut conn, tag_data.into_inner().into())?;
    let response = tag.to_response(&mut conn)?;

    Ok(HttpResponse::Created().json(response))
}

/// Gets a tag by ID
#[utoipa::path(
    get,
    path = "/api/tags/{id}",
    params(TagPath),
    responses(
        (status = 200, description = "Tag found", body = TagResponse),
        (status = 404, description = "Tag not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Tags"
)]
pub async fn get_tag(
    pool: web::Data<DbPool>,
    path: web::Path<TagPath>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    let tag = Tag::find_by_id(&mut conn, path.id)?;
    let response = tag.to_response(&mut conn)?;

    Ok(HttpResponse::Ok().json(response))
}

/// Lists tags with pagination
#[utoipa::path(
    get,
    path = "/api/tags",
    params(TagListQuery),
    responses(
        (status = 200, description = "Tags retrieved successfully", body = TagListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Tags"
)]
pub async fn list_tags(
    pool: web::Data<DbPool>,
    query: web::Query<TagListQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate and set defaults for pagination
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100).max(1);

    let (tags, total) = Tag::list_paginated(&mut conn, page, per_page)?;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    
    let mut tag_responses = Vec::new();
    for tag in tags {
        tag_responses.push(tag.to_response(&mut conn)?);
    }

    let response = TagListResponse {
        tags: tag_responses,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Gets popular tags
#[utoipa::path(
    get,
    path = "/api/tags/popular",
    params(PopularTagsQuery),
    responses(
        (status = 200, description = "Popular tags retrieved successfully", body = Vec<PopularTagResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Tags"
)]
pub async fn get_popular_tags(
    pool: web::Data<DbPool>,
    query: web::Query<PopularTagsQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    let limit = query.limit.unwrap_or(10).min(50).max(1);
    let popular_tags = Tag::get_popular(&mut conn, limit)?;

    Ok(HttpResponse::Ok().json(popular_tags))
}

/// Updates a tag
#[utoipa::path(
    put,
    path = "/api/tags/{id}",
    params(TagPath),
    request_body = UpdateTag,
    responses(
        (status = 200, description = "Tag updated successfully", body = TagResponse),
        (status = 400, description = "Tag name already exists", body = ErrorResponse),
        (status = 404, description = "Tag not found", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Tags"
)]
pub async fn update_tag(
    pool: web::Data<DbPool>,
    path: web::Path<TagPath>,
    update_data: web::Json<UpdateTag>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate update data
    if let Some(ref name) = update_data.name {
        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Tag name cannot be empty".to_string()));
        }
    }

    let tag = Tag::update(&mut conn, path.id, update_data.into_inner())?;
    
    // Update usage count after update
    tag.update_usage_count(&mut conn)?;
    
    let response = tag.to_response(&mut conn)?;

    Ok(HttpResponse::Ok().json(response))
}

/// Soft deletes a tag
#[utoipa::path(
    delete,
    path = "/api/tags/{id}",
    params(TagPath),
    responses(
        (status = 204, description = "Tag deleted successfully"),
        (status = 404, description = "Tag not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Tags"
)]
pub async fn delete_tag(
    pool: web::Data<DbPool>,
    path: web::Path<TagPath>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    Tag::soft_delete(&mut conn, path.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    // Unit tests can be added here
}