# Database Design Document

> **Document Info**  
> Version: v2.1  
> Last Updated: 2025-06-25  
> Author: Development Team  
> Status: Final

## 📋 Table of Contents

- [Design Principles](#design-principles)
- [ID Type Selection](#id-type-selection)
- [Database Schema](#database-schema)
- [View Definitions](#view-definitions)
- [Trigger Implementation](#trigger-implementation)
- [Soft Delete Implementation](#soft-delete-implementation)
- [Performance Optimization](#performance-optimization)
- [Data Integrity](#data-integrity)
- [Junction Table Design](#junction-table-design)

## 🎯 Design Principles

### Core Design Philosophy
1. **Table Structure Separation**: Frequently updated attributes in separate tables to reduce main table update frequency
2. **Soft Delete Pattern**: All tables support soft delete to preserve data history
3. **Flexible Tagging System**: Independent tag tables for flexible classification management
4. **Performance Optimization**: Strategic indexing to reduce table locking
5. **BIGINT Primary Keys**: Use BIGINT for primary keys to support large datasets with excellent index performance

### Key Features
- ✅ **Soft Delete Support**: All data is preserved with timestamp-based deletion
- ✅ **Hierarchical Categories**: Support for nested category structures
- ✅ **Flexible Tagging**: Many-to-many relationships for books and notes
- ✅ **Full-Text Search**: PostgreSQL GIN indexes for content search
- ✅ **Automatic Timestamps**: Created/updated timestamps with triggers

## 🔢 ID Type Selection

### Why BIGINT Primary Keys

#### Advantages
1. **Large Capacity**: Supports -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
2. **Excellent Index Performance**: Integer types perform optimally in B-tree indexes
3. **Compact Storage**: 8 bytes vs 16 bytes for UUID
4. **Sequential Ordering**: SERIAL auto-increment ensures insertion order, reduces page splits
5. **Universal Compatibility**: Native support in all ORM frameworks

#### Junction Table Optimization
- Junction tables use composite primary keys instead of separate ID fields
- Reduces storage space and improves query efficiency
- Natural prevention of duplicate associations

## 🗄️ Database Schema

### 1. Books Table
Stores core book information that rarely changes.

```sql
CREATE TABLE books (
    id BIGSERIAL PRIMARY KEY,                      -- Auto-incrementing BIGINT primary key
    isbn VARCHAR(20),                              -- ISBN number (optional)
    title VARCHAR(200) NOT NULL,                   -- Book title
    author VARCHAR(100) NOT NULL,                  -- Author name
    publisher VARCHAR(100),                        -- Publisher
    publication_date DATE,                         -- Publication date
    page_count INTEGER,                            -- Total page count
    cover_image VARCHAR(255),                      -- Cover image URL
    description TEXT,                              -- Book description
    deleted_at TIMESTAMPTZ,                        -- Soft delete timestamp (with timezone)
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Indexes (partial indexes for non-deleted records only)
CREATE INDEX idx_books_title ON books(title) WHERE deleted_at IS NULL;
CREATE INDEX idx_books_author ON books(author) WHERE deleted_at IS NULL;
CREATE INDEX idx_books_isbn ON books(isbn) WHERE deleted_at IS NULL;
CREATE INDEX idx_books_deleted_at ON books(deleted_at);
```

### 2. Reading Notes Table
Stores user reading notes with full-text search capability.

```sql
CREATE TABLE reading_notes (
    id BIGSERIAL PRIMARY KEY,
    book_id BIGINT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    title VARCHAR(200),                            -- Note title
    content TEXT NOT NULL,                         -- Note content (Markdown supported)
    note_type VARCHAR(20) DEFAULT 'general',      -- Note types: quote/summary/thought/general
    page_reference INTEGER,                       -- Page reference (simplified to integer)
    is_favorite BOOLEAN DEFAULT FALSE,             -- Favorite flag
    deleted_at TIMESTAMPTZ,                        -- Soft delete timestamp
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_notes_book_id ON reading_notes(book_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_notes_created_at ON reading_notes(created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_notes_is_favorite ON reading_notes(is_favorite) WHERE deleted_at IS NULL AND is_favorite = TRUE;
CREATE INDEX idx_notes_note_type ON reading_notes(note_type) WHERE deleted_at IS NULL;

-- Full-text search index
CREATE INDEX idx_notes_fulltext ON reading_notes 
    USING gin(to_tsvector('simple', title || ' ' || content)) 
    WHERE deleted_at IS NULL;
```

### 3. Tags Table
Flexible tagging system with usage tracking.

```sql
CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,                     -- Tag name
    slug VARCHAR(30) NOT NULL,                     -- URL-friendly identifier
    description TEXT,                              -- Tag description (optional)
    usage_count INTEGER DEFAULT 0,                 -- Usage count for popularity
    deleted_at TIMESTAMPTZ,                        -- Soft delete timestamp
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(slug) WHERE deleted_at IS NULL
);

-- Indexes
CREATE INDEX idx_tags_slug ON tags(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_tags_usage_count ON tags(usage_count DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_tags_name ON tags(name) WHERE deleted_at IS NULL;
```

### 4. Note-Tag Associations Table
Many-to-many relationship between notes and tags using composite primary key.

```sql
CREATE TABLE note_tag_associations (
    note_id BIGINT NOT NULL REFERENCES reading_notes(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    deleted_at TIMESTAMPTZ,                        -- Soft delete timestamp
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (note_id, tag_id)                  -- Composite primary key
);

-- Indexes
CREATE INDEX idx_note_tags_tag_id ON note_tag_associations(tag_id, note_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_note_tags_deleted_at ON note_tag_associations(deleted_at) WHERE deleted_at IS NOT NULL;
```

## 📊 View Definitions

### 1. Notes with Tags View
Aggregates notes with their associated tags for efficient querying.

```sql
CREATE VIEW v_notes_with_tags AS
SELECT 
    n.id,
    n.book_id,
    n.title,
    n.content,
    n.note_type,
    n.page_reference,
    n.is_favorite,
    n.created_at,
    n.updated_at,
    b.title as book_title,
    b.author as book_author,
    COALESCE(
        ARRAY_AGG(t.name ORDER BY t.name) FILTER (WHERE t.name IS NOT NULL), 
        ARRAY[]::VARCHAR[]
    ) as tags
FROM reading_notes n
LEFT JOIN books b ON n.book_id = b.id AND b.deleted_at IS NULL
LEFT JOIN note_tag_associations nta ON n.id = nta.note_id AND nta.deleted_at IS NULL
LEFT JOIN tags t ON nta.tag_id = t.id AND t.deleted_at IS NULL
WHERE n.deleted_at IS NULL
GROUP BY n.id, n.book_id, n.title, n.content, n.note_type, n.page_reference, 
         n.is_favorite, n.created_at, n.updated_at, b.title, b.author;
```

### 2. Popular Tags View
Shows tags ordered by usage count for analytics.

```sql
CREATE VIEW v_popular_tags AS
SELECT 
    t.id,
    t.name,
    t.slug,
    t.description,
    t.usage_count,
    t.created_at
FROM tags t
WHERE t.deleted_at IS NULL
  AND t.usage_count > 0
ORDER BY t.usage_count DESC, t.name ASC;
```

## ⚡ Trigger Implementation

### 1. Automatic Timestamp Updates
```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

-- Create triggers for tables that need automatic timestamp updates
CREATE TRIGGER update_books_updated_at 
    BEFORE UPDATE ON books
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reading_notes_updated_at 
    BEFORE UPDATE ON reading_notes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tags_updated_at 
    BEFORE UPDATE ON tags
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

### 2. Tag Usage Count Management
```sql
CREATE OR REPLACE FUNCTION update_tag_usage_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' AND NEW.deleted_at IS NULL THEN
        UPDATE tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
    ELSIF TG_OP = 'UPDATE' THEN
        -- Handle soft delete/restore
        IF OLD.deleted_at IS NULL AND NEW.deleted_at IS NOT NULL THEN
            UPDATE tags SET usage_count = usage_count - 1 WHERE id = NEW.tag_id;
        ELSIF OLD.deleted_at IS NOT NULL AND NEW.deleted_at IS NULL THEN
            UPDATE tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE tags SET usage_count = usage_count - 1 WHERE id = OLD.tag_id;
    END IF;
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_tag_count_on_note_tags
    AFTER INSERT OR UPDATE OR DELETE ON note_tag_associations
    FOR EACH ROW EXECUTE FUNCTION update_tag_usage_count();
```

## 🗑️ Soft Delete Implementation

### 1. Soft Delete Utility Functions
```sql
-- Generic soft delete function
CREATE OR REPLACE FUNCTION soft_delete(table_name text, record_id bigint)
RETURNS void AS $$
BEGIN
    EXECUTE format('UPDATE %I SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1', table_name)
    USING record_id;
END;
$$ LANGUAGE plpgsql;

-- Batch soft delete function
CREATE OR REPLACE FUNCTION soft_delete_batch(table_name text, record_ids bigint[])
RETURNS void AS $$
BEGIN
    EXECUTE format('UPDATE %I SET deleted_at = CURRENT_TIMESTAMP WHERE id = ANY($1)', table_name)
    USING record_ids;
END;
$$ LANGUAGE plpgsql;

-- Restore soft deleted records
CREATE OR REPLACE FUNCTION restore_deleted(table_name text, record_id bigint)
RETURNS void AS $$
BEGIN
    EXECUTE format('UPDATE %I SET deleted_at = NULL WHERE id = $1', table_name)
    USING record_id;
END;
$$ LANGUAGE plpgsql;
```

### 2. Cascading Soft Delete for Books
```sql
CREATE OR REPLACE FUNCTION cascade_soft_delete_book()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL THEN
        -- Soft delete related notes
        UPDATE reading_notes SET deleted_at = NEW.deleted_at 
        WHERE book_id = NEW.id AND deleted_at IS NULL;
        
        -- Soft delete note-tag associations for this book's notes
        UPDATE note_tag_associations SET deleted_at = NEW.deleted_at 
        WHERE note_id IN (
            SELECT id FROM reading_notes 
            WHERE book_id = NEW.id AND deleted_at IS NULL
        ) AND deleted_at IS NULL;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER cascade_delete_book_relations
    AFTER UPDATE OF deleted_at ON books
    FOR EACH ROW
    EXECUTE FUNCTION cascade_soft_delete_book();
```

## 🚀 Performance Optimization

### Indexing Strategy
1. **Partial Indexes**: All indexes use `WHERE deleted_at IS NULL` to index only active records
2. **Composite Indexes**: Junction tables use composite primary keys for optimal query performance
3. **Full-Text Search**: GIN indexes for content search with proper text vectors
4. **Usage-Based Indexes**: Indexes on frequently queried columns like `usage_count`, `created_at`

### Query Optimization Tips
1. **Always Include Soft Delete Filter**: Every query should include `WHERE deleted_at IS NULL`
2. **Use Views for Complex Joins**: Pre-defined views for common query patterns
3. **Pagination Support**: All list queries should support LIMIT/OFFSET
4. **Index Maintenance**: Regular VACUUM and ANALYZE for index health

### Recommended Settings
```sql
-- For better full-text search performance
SET default_text_search_config = 'simple';

-- For better timestamp precision
SET timezone = 'UTC';
```

## 🔒 Data Integrity

### Constraints and Validation
1. **Unique Constraints**: Conditional unique constraints for soft-deleted environment
2. **Foreign Key Constraints**: Maintain referential integrity across tables
3. **Check Constraints**: Validate enum values and data ranges
4. **Triggers**: Automatic maintenance of data consistency

### Transaction Safety
- All related operations execute within transactions
- Cascading operations handled through triggers
- Atomic soft delete operations
- Consistent timestamp handling

## 🔗 Junction Table Design

### Composite Primary Key Benefits

#### Why Use Composite Keys for Associations
1. **Storage Efficiency**: No additional ID field needed, saves storage space
2. **Query Performance**: Composite primary key automatically creates unique index
3. **Data Integrity**: Natural prevention of duplicate associations
4. **Simplified Queries**: Direct JOIN operations without extra ID field overhead

#### Index Strategy for Junction Tables
```sql
-- Example for note_tag_associations
PRIMARY KEY (note_id, tag_id)           -- Forward lookup: note → tags
CREATE INDEX (tag_id, note_id)          -- Reverse lookup: tag → notes
CREATE INDEX (deleted_at) WHERE deleted_at IS NOT NULL  -- Cleanup queries
```

### Junction Table Patterns
- Use composite primary keys for all many-to-many relationships
- Include `deleted_at` for soft delete support
- Create reverse indexes for bidirectional queries
- Separate indexes for cleanup operations

## 📈 Scalability Considerations

### Future Enhancements
1. **Table Partitioning**: Partition `reading_notes` by date if volume grows large
2. **Read Replicas**: Separate read queries for analytics and reporting
3. **Async Updates**: Background updates for statistics and counts
4. **Archival Strategy**: Move old soft-deleted records to archive tables

### Monitoring Recommendations
- Monitor index usage with `pg_stat_user_indexes`
- Track query performance with `pg_stat_statements`
- Regular cleanup of old soft-deleted records
- Monitor tag usage count accuracy

---

**Document Version**: v2.1  
**Next Review**: 2025-07-25  
**Database Schema**: Production Ready