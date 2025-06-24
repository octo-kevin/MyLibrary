-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Books table
CREATE TABLE books (
    id BIGSERIAL PRIMARY KEY,
    isbn VARCHAR(20),
    title VARCHAR(200) NOT NULL,
    author VARCHAR(100) NOT NULL,
    publisher VARCHAR(100),
    publication_date DATE,
    page_count INTEGER,
    cover_image VARCHAR(255),
    description TEXT,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. Reading status table
CREATE TABLE reading_status (
    id BIGSERIAL PRIMARY KEY,
    book_id BIGINT NOT NULL REFERENCES books(id),
    status VARCHAR(20) NOT NULL DEFAULT 'to_read',
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    start_date DATE,
    finish_date DATE,
    current_page INTEGER DEFAULT 0,
    reading_progress DECIMAL(5,2) DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 3. Categories table
CREATE TABLE categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    slug VARCHAR(50) NOT NULL,
    color VARCHAR(7),
    description TEXT,
    parent_id BIGINT REFERENCES categories(id),
    sort_order INTEGER DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 4. Book categories association table
CREATE TABLE book_categories (
    book_id BIGINT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (book_id, category_id)
);

-- 5. Tags table
CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    slug VARCHAR(30) NOT NULL,
    usage_count INTEGER DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 6. Book tags association table
CREATE TABLE book_tags (
    book_id BIGINT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (book_id, tag_id)
);

-- 7. Reading notes table
CREATE TABLE reading_notes (
    id BIGSERIAL PRIMARY KEY,
    book_id BIGINT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    title VARCHAR(200),
    content TEXT NOT NULL,
    note_type VARCHAR(20) DEFAULT 'general',
    page_reference VARCHAR(50),
    is_favorite BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 8. Note tags association table
CREATE TABLE note_tags (
    note_id BIGINT NOT NULL REFERENCES reading_notes(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (note_id, tag_id)
);

-- Create indexes
CREATE INDEX idx_books_title ON books(title) WHERE deleted_at IS NULL;
CREATE INDEX idx_books_author ON books(author) WHERE deleted_at IS NULL;
CREATE INDEX idx_books_isbn ON books(isbn) WHERE deleted_at IS NULL;
CREATE INDEX idx_books_deleted_at ON books(deleted_at);

-- Create unique indexes for soft delete support
CREATE UNIQUE INDEX idx_reading_status_book_id_unique ON reading_status(book_id) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX idx_categories_slug_unique ON categories(slug) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX idx_tags_slug_unique ON tags(slug) WHERE deleted_at IS NULL;

CREATE INDEX idx_reading_status_book_id ON reading_status(book_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_reading_status_status ON reading_status(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_reading_status_finish_date ON reading_status(finish_date DESC) WHERE deleted_at IS NULL;

CREATE INDEX idx_categories_parent_id ON categories(parent_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_categories_slug ON categories(slug) WHERE deleted_at IS NULL;

CREATE INDEX idx_book_categories_category_id ON book_categories(category_id, book_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_book_categories_deleted_at ON book_categories(deleted_at) WHERE deleted_at IS NOT NULL;

CREATE INDEX idx_tags_slug ON tags(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_tags_usage_count ON tags(usage_count DESC) WHERE deleted_at IS NULL;

CREATE INDEX idx_book_tags_tag_id ON book_tags(tag_id, book_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_book_tags_deleted_at ON book_tags(deleted_at) WHERE deleted_at IS NOT NULL;

CREATE INDEX idx_notes_book_id ON reading_notes(book_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_notes_created_at ON reading_notes(created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_notes_is_favorite ON reading_notes(is_favorite) WHERE deleted_at IS NULL AND is_favorite = TRUE;

CREATE INDEX idx_note_tags_tag_id ON note_tags(tag_id, note_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_note_tags_deleted_at ON note_tags(deleted_at) WHERE deleted_at IS NOT NULL;

-- Full text search indexes
CREATE INDEX idx_books_fulltext ON books 
    USING gin(to_tsvector('simple', title || ' ' || author)) 
    WHERE deleted_at IS NULL;

CREATE INDEX idx_notes_fulltext ON reading_notes 
    USING gin(to_tsvector('simple', COALESCE(title, '') || ' ' || content)) 
    WHERE deleted_at IS NULL;

-- Create update timestamp function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_books_updated_at BEFORE UPDATE ON books
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reading_status_updated_at BEFORE UPDATE ON reading_status
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reading_notes_updated_at BEFORE UPDATE ON reading_notes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Create tag usage count update function
CREATE OR REPLACE FUNCTION update_tag_usage_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE tags SET usage_count = usage_count - 1 WHERE id = OLD.tag_id;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

-- Create triggers for tag usage count
CREATE TRIGGER update_tag_count_on_book_tags
    AFTER INSERT OR DELETE ON book_tags
    FOR EACH ROW EXECUTE FUNCTION update_tag_usage_count();

CREATE TRIGGER update_tag_count_on_note_tags
    AFTER INSERT OR DELETE ON note_tags
    FOR EACH ROW EXECUTE FUNCTION update_tag_usage_count();