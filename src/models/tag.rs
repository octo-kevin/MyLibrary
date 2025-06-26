use crate::db::schema::{book_tags, note_tags, tags};
use crate::errors::{AppError, Result};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Tag database model
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub usage_count: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

/// New tag for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
    pub slug: String,
}

/// Update tag structure
#[derive(Debug, Deserialize, AsChangeset, Default, ToSchema)]
#[diesel(table_name = tags)]
pub struct UpdateTag {
    #[schema(example = "Updated Tag")]
    pub name: Option<String>,
    #[schema(example = "updated-tag")]
    pub slug: Option<String>,
}

/// Request structure for creating a new tag
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTagRequest {
    #[schema(example = "Important")]
    pub name: String,
}

/// Response structure for tag
#[derive(Debug, Serialize, ToSchema)]
pub struct TagResponse {
    #[schema(example = 1)]
    pub id: i64,

    #[schema(example = "Important")]
    pub name: String,

    #[schema(example = "important")]
    pub slug: String,

    #[schema(example = 10)]
    pub book_count: i64,

    #[schema(example = 25)]
    pub note_count: i64,

    #[schema(example = 35)]
    pub usage_count: i32,

    #[schema(example = "2024-01-01T12:00:00Z")]
    pub created_at: Option<DateTime<Utc>>,
}

/// Paginated tag list response
#[derive(Debug, Serialize, ToSchema)]
pub struct TagListResponse {
    pub tags: Vec<TagResponse>,

    #[schema(example = 50)]
    pub total: i64,

    #[schema(example = 1)]
    pub page: u32,

    #[schema(example = 20)]
    pub per_page: u32,

    #[schema(example = 3)]
    pub total_pages: u32,
}

/// Popular tag response
#[derive(Debug, Serialize, ToSchema)]
pub struct PopularTagResponse {
    #[schema(example = 1)]
    pub id: i64,

    #[schema(example = "Important")]
    pub name: String,

    #[schema(example = "important")]
    pub slug: String,

    #[schema(example = 35)]
    pub usage_count: i32,
}

impl From<CreateTagRequest> for NewTag {
    fn from(req: CreateTagRequest) -> Self {
        let name = req.name.trim().to_string();
        let slug = slugify(&name);
        Self { name, slug }
    }
}

/// Convert a string to a URL-safe slug
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

impl Tag {
    /// Creates a new tag
    pub fn create(conn: &mut PgConnection, new_tag: NewTag) -> Result<Tag> {
        // Check if tag with same slug already exists
        let existing = tags::table
            .filter(tags::slug.eq(&new_tag.slug))
            .filter(tags::deleted_at.is_null())
            .first::<Tag>(conn)
            .optional()?;

        if let Some(tag) = existing {
            return Err(AppError::BadRequest(format!(
                "Tag '{}' already exists",
                tag.name
            )));
        }

        diesel::insert_into(tags::table)
            .values(&new_tag)
            .returning(Tag::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    /// Finds a tag by ID (excluding soft deleted)
    pub fn find_by_id(conn: &mut PgConnection, tag_id: i64) -> Result<Tag> {
        tags::table
            .filter(tags::id.eq(tag_id))
            .filter(tags::deleted_at.is_null())
            .first(conn)
            .map_err(|_| AppError::NotFound(format!("Tag with id {} not found", tag_id)))
    }

    /// Finds a tag by slug
    pub fn find_by_slug(conn: &mut PgConnection, slug: &str) -> Result<Option<Tag>> {
        tags::table
            .filter(tags::slug.eq(slug))
            .filter(tags::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(AppError::from)
    }

    /// Finds or creates a tag by name
    pub fn find_or_create(conn: &mut PgConnection, name: String) -> Result<Tag> {
        let slug = slugify(&name);

        // First try to find existing tag by slug
        if let Some(tag) = Self::find_by_slug(conn, &slug)? {
            return Ok(tag);
        }

        // Create new tag if not found
        let new_tag = NewTag {
            name: name.trim().to_string(),
            slug,
        };
        Self::create(conn, new_tag)
    }

    /// Lists all tags with pagination
    pub fn list_paginated(
        conn: &mut PgConnection,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Tag>, i64)> {
        let offset = ((page.saturating_sub(1)) * per_page) as i64;

        let tags = tags::table
            .filter(tags::deleted_at.is_null())
            .order(tags::name.asc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<Tag>(conn)?;

        let total = tags::table
            .filter(tags::deleted_at.is_null())
            .count()
            .get_result::<i64>(conn)?;

        Ok((tags, total))
    }

    /// Lists tags with optional search filter
    pub fn list_with_search(
        conn: &mut PgConnection,
        search_query: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Tag>, i64)> {
        let offset = ((page.saturating_sub(1)) * per_page) as i64;

        // Prepare search pattern if needed
        let search_pattern = search_query
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("%{}%", s.trim()));

        let mut query = tags::table.filter(tags::deleted_at.is_null()).into_boxed();

        // Apply search filter if provided
        if let Some(ref pattern) = search_pattern {
            query = query.filter(tags::name.ilike(pattern));
        }

        let tags = query
            .order(tags::name.asc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<Tag>(conn)?;

        // Get total count with the same search filter
        let mut count_query = tags::table.filter(tags::deleted_at.is_null()).into_boxed();

        if let Some(ref pattern) = search_pattern {
            count_query = count_query.filter(tags::name.ilike(pattern));
        }

        let total = count_query.count().get_result::<i64>(conn)?;

        Ok((tags, total))
    }

    /// Gets popular tags sorted by usage count
    pub fn get_popular(conn: &mut PgConnection, limit: i64) -> Result<Vec<PopularTagResponse>> {
        let popular_tags = tags::table
            .filter(tags::deleted_at.is_null())
            .filter(tags::usage_count.is_not_null())
            .order(tags::usage_count.desc())
            .limit(limit)
            .load::<Tag>(conn)?;

        Ok(popular_tags
            .into_iter()
            .map(|tag| PopularTagResponse {
                id: tag.id,
                name: tag.name,
                slug: tag.slug,
                usage_count: tag.usage_count.unwrap_or(0),
            })
            .collect())
    }

    /// Updates a tag
    pub fn update(conn: &mut PgConnection, tag_id: i64, update_data: UpdateTag) -> Result<Tag> {
        // If updating name, also update slug
        let mut update_data = update_data;
        if let Some(ref name) = update_data.name {
            update_data.slug = Some(slugify(name));

            // Check for duplicates
            if let Some(ref slug) = update_data.slug {
                let existing = tags::table
                    .filter(tags::slug.eq(slug))
                    .filter(tags::id.ne(tag_id))
                    .filter(tags::deleted_at.is_null())
                    .first::<Tag>(conn)
                    .optional()?;

                if existing.is_some() {
                    return Err(AppError::BadRequest(format!(
                        "Tag '{}' already exists",
                        name
                    )));
                }
            }
        }

        diesel::update(tags::table.find(tag_id))
            .filter(tags::deleted_at.is_null())
            .set(&update_data)
            .returning(Tag::as_returning())
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Tag with id {} not found", tag_id))
                }
                _ => AppError::from(e),
            })
    }

    /// Soft deletes a tag
    pub fn soft_delete(conn: &mut PgConnection, tag_id: i64) -> Result<()> {
        let affected = diesel::update(tags::table.find(tag_id))
            .filter(tags::deleted_at.is_null())
            .set(tags::deleted_at.eq(Some(Utc::now())))
            .execute(conn)?;

        if affected == 0 {
            return Err(AppError::NotFound(format!(
                "Tag with id {} not found",
                tag_id
            )));
        }

        Ok(())
    }

    /// Gets the count of books using this tag
    pub fn get_book_count(&self, conn: &mut PgConnection) -> Result<i64> {
        use crate::db::schema::books;

        book_tags::table
            .inner_join(books::table)
            .filter(book_tags::tag_id.eq(self.id))
            .filter(books::deleted_at.is_null())
            .count()
            .get_result(conn)
            .map_err(AppError::from)
    }

    /// Gets the count of notes using this tag
    pub fn get_note_count(&self, conn: &mut PgConnection) -> Result<i64> {
        use crate::db::schema::reading_notes;

        note_tags::table
            .inner_join(reading_notes::table)
            .filter(note_tags::tag_id.eq(self.id))
            .filter(reading_notes::deleted_at.is_null())
            .count()
            .get_result(conn)
            .map_err(AppError::from)
    }

    /// Updates the usage count for this tag
    pub fn update_usage_count(&self, conn: &mut PgConnection) -> Result<()> {
        let book_count = self.get_book_count(conn)?;
        let note_count = self.get_note_count(conn)?;
        let total_count = (book_count + note_count) as i32;

        diesel::update(tags::table.find(self.id))
            .set(tags::usage_count.eq(Some(total_count)))
            .execute(conn)?;

        Ok(())
    }
}

/// Converts Tag to TagResponse with usage counts
impl Tag {
    pub fn to_response(&self, conn: &mut PgConnection) -> Result<TagResponse> {
        let book_count = self.get_book_count(conn)?;
        let note_count = self.get_note_count(conn)?;

        Ok(TagResponse {
            id: self.id,
            name: self.name.clone(),
            slug: self.slug.clone(),
            book_count,
            note_count,
            usage_count: self.usage_count.unwrap_or(0),
            created_at: self.created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Test  Multiple   Spaces"), "test-multiple-spaces");
        assert_eq!(slugify("Special!@#Characters"), "special-characters");
        assert_eq!(slugify("  Trim Spaces  "), "trim-spaces");
    }

    #[test]
    fn test_tag_creation_from_request() {
        let req = CreateTagRequest {
            name: "  New Tag!  ".to_string(),
        };
        let new_tag: NewTag = req.into();
        assert_eq!(new_tag.name, "New Tag!");
        assert_eq!(new_tag.slug, "new-tag");
    }
}
