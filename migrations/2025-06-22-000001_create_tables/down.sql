-- Drop triggers first
DROP TRIGGER IF EXISTS update_tag_count_on_note_tags ON note_tags;
DROP TRIGGER IF EXISTS update_tag_count_on_book_tags ON book_tags;
DROP TRIGGER IF EXISTS update_reading_notes_updated_at ON reading_notes;
DROP TRIGGER IF EXISTS update_categories_updated_at ON categories;
DROP TRIGGER IF EXISTS update_reading_status_updated_at ON reading_status;
DROP TRIGGER IF EXISTS update_books_updated_at ON books;

-- Drop functions
DROP FUNCTION IF EXISTS update_tag_usage_count();
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS note_tags;
DROP TABLE IF EXISTS reading_notes;
DROP TABLE IF EXISTS book_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS book_categories;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS reading_status;
DROP TABLE IF EXISTS books;