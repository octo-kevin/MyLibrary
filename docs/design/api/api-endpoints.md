# API Endpoints Reference

> **Document Info**  
> Version: v2.0  
> Last Updated: 2025-06-25  
> Author: Development Team  
> Status: Production Ready

## 📋 Table of Contents

- [API Overview](#api-overview)
- [Books API](#books-api)
- [Notes API](#notes-api)
- [Tags API](#tags-api)
- [Response Formats](#response-formats)
- [Error Handling](#error-handling)
- [Pagination](#pagination)

## 🔗 API Overview

### Base Information
- **Base URL**: `http://localhost:8080/api`
- **Content-Type**: `application/json`
- **Response Format**: JSON
- **Authentication**: None (single-user system)
- **API Version**: v1
- **Total Endpoints**: 19

### HTTP Status Codes
- `200 OK` - Successful GET, PUT requests
- `201 Created` - Successful POST requests
- `204 No Content` - Successful DELETE requests
- `400 Bad Request` - Invalid request data
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## 📚 Books API

### 1. Create Book
Creates a new book record.

**Endpoint**: `POST /api/books`

**Request Body**:
```json
{
  "title": "The Rust Programming Language",
  "author": "Steve Klabnik",
  "isbn": "978-1593278281",
  "publisher": "No Starch Press",
  "page_count": 552,
  "description": "The official guide to Rust programming"
}
```

**Response** (201 Created):
```json
{
  "id": 1,
  "title": "The Rust Programming Language",
  "author": "Steve Klabnik", 
  "isbn": "978-1593278281",
  "publisher": "No Starch Press",
  "page_count": 552,
  "description": "The official guide to Rust programming",
  "created_at": "2025-01-01T12:00:00Z",
  "updated_at": "2025-01-01T12:00:00Z"
}
```

### 2. Get Books List
Retrieves a paginated list of books with optional search.

**Endpoint**: `GET /api/books`

**Query Parameters**:
- `page` (integer, optional): Page number (default: 1)
- `per_page` (integer, optional): Items per page (default: 20, max: 100)
- `search` (string, optional): Search in title and author

**Example Request**:
```
GET /api/books?page=1&per_page=10&search=rust
```

**Response** (200 OK):
```json
{
  "books": [
    {
      "id": 1,
      "title": "The Rust Programming Language",
      "author": "Steve Klabnik",
      "isbn": "978-1593278281", 
      "publisher": "No Starch Press",
      "page_count": 552,
      "description": "The official guide to Rust programming",
      "created_at": "2025-01-01T12:00:00Z",
      "updated_at": "2025-01-01T12:00:00Z"
    }
  ],
  "total": 50,
  "page": 1,
  "per_page": 10,
  "total_pages": 5
}
```

### 3. Get Book Details
Retrieves detailed information for a specific book.

**Endpoint**: `GET /api/books/{id}`

**Response** (200 OK):
```json
{
  "id": 1,
  "title": "The Rust Programming Language",
  "author": "Steve Klabnik",
  "isbn": "978-1593278281",
  "publisher": "No Starch Press", 
  "page_count": 552,
  "description": "The official guide to Rust programming",
  "created_at": "2025-01-01T12:00:00Z",
  "updated_at": "2025-01-01T12:00:00Z"
}
```

### 4. Update Book
Updates an existing book record.

**Endpoint**: `PUT /api/books/{id}`

**Request Body** (partial updates allowed):
```json
{
  "title": "Updated Book Title",
  "description": "Updated description"
}
```

**Response** (200 OK): Updated book object

### 5. Delete Book
Soft deletes a book record.

**Endpoint**: `DELETE /api/books/{id}`

**Response** (204 No Content): Empty body

### 6. Get Book Notes
Retrieves all notes associated with a specific book.

**Endpoint**: `GET /api/books/{id}/notes`

**Query Parameters**:
- `page` (integer, optional): Page number (default: 1)
- `per_page` (integer, optional): Items per page (default: 20)

**Response** (200 OK):
```json
{
  "notes": [
    {
      "id": 1,
      "title": "Chapter 1 Summary",
      "content": "Rust is a systems programming language...",
      "note_type": "summary",
      "page_reference": 15,
      "is_favorite": false,
      "tags": ["programming", "rust"],
      "created_at": "2025-01-02T10:00:00Z",
      "updated_at": "2025-01-02T10:00:00Z"
    }
  ],
  "total": 25,
  "page": 1,
  "per_page": 20,
  "total_pages": 2
}
```

## 📝 Notes API

### 1. Create Note
Creates a new reading note.

**Endpoint**: `POST /api/notes`

**Request Body**:
```json
{
  "title": "Chapter 1 Summary",
  "content": "Rust is a systems programming language focused on safety...",
  "note_type": "summary",
  "book_id": 1,
  "page_reference": 15,
  "is_favorite": false,
  "tags": ["programming", "rust", "systems"]
}
```

**Note Types**:
- `quote` - Direct quotations from the book
- `summary` - Chapter or section summaries
- `thought` - Personal thoughts and reflections
- `general` - General notes

**Response** (201 Created):
```json
{
  "id": 1,
  "title": "Chapter 1 Summary", 
  "content": "Rust is a systems programming language focused on safety...",
  "note_type": "summary",
  "book_id": 1,
  "page_reference": 15,
  "is_favorite": false,
  "tags": ["programming", "rust", "systems"],
  "book": {
    "id": 1,
    "title": "The Rust Programming Language",
    "author": "Steve Klabnik"
  },
  "created_at": "2025-01-02T10:00:00Z",
  "updated_at": "2025-01-02T10:00:00Z"
}
```

### 2. Get Notes List
Retrieves a paginated list of notes with filtering options.

**Endpoint**: `GET /api/notes`

**Query Parameters**:
- `page` (integer, optional): Page number (default: 1)
- `per_page` (integer, optional): Items per page (default: 20)
- `search` (string, optional): Search in title and content
- `note_type` (string, optional): Filter by note type
- `book_id` (integer, optional): Filter by book ID

**Example Request**:
```
GET /api/notes?page=1&per_page=10&search=rust&note_type=summary
```

**Response** (200 OK):
```json
{
  "notes": [
    {
      "id": 1,
      "title": "Chapter 1 Summary",
      "content": "Rust is a systems programming language...",
      "note_type": "summary", 
      "page_reference": 15,
      "is_favorite": false,
      "tags": ["programming", "rust"],
      "book": {
        "id": 1,
        "title": "The Rust Programming Language",
        "author": "Steve Klabnik"
      },
      "created_at": "2025-01-02T10:00:00Z",
      "updated_at": "2025-01-02T10:00:00Z"
    }
  ],
  "total": 120,
  "page": 1,
  "per_page": 10,
  "total_pages": 12
}
```

### 3. Get Note Details
Retrieves detailed information for a specific note.

**Endpoint**: `GET /api/notes/{id}`

**Response** (200 OK): Single note object with full details

### 4. Update Note
Updates an existing note.

**Endpoint**: `PUT /api/notes/{id}`

**Request Body** (partial updates allowed):
```json
{
  "title": "Updated Note Title",
  "content": "Updated note content...",
  "is_favorite": true
}
```

**Response** (200 OK): Updated note object

### 5. Delete Note  
Soft deletes a note record.

**Endpoint**: `DELETE /api/notes/{id}`

**Response** (204 No Content): Empty body

### 6. Update Note Tags
Updates the tags associated with a note.

**Endpoint**: `PUT /api/notes/{id}/tags`

**Request Body**:
```json
{
  "tags": ["programming", "rust", "systems", "new-tag"]
}
```

**Response** (200 OK): Updated note object with new tags

## 🏷️ Tags API

### 1. Create Tag
Creates a new tag.

**Endpoint**: `POST /api/tags`

**Request Body**:
```json
{
  "name": "Programming",
  "description": "Notes about programming concepts"
}
```

**Response** (201 Created):
```json
{
  "id": 1,
  "name": "Programming", 
  "slug": "programming",
  "description": "Notes about programming concepts",
  "usage_count": 0,
  "created_at": "2025-01-01T12:00:00Z",
  "updated_at": "2025-01-01T12:00:00Z"
}
```

### 2. Get Tags List
Retrieves a paginated list of tags with optional search.

**Endpoint**: `GET /api/tags`

**Query Parameters**:
- `page` (integer, optional): Page number (default: 1)
- `per_page` (integer, optional): Items per page (default: 20)
- `search` (string, optional): Search in tag names

**Example Request**:
```
GET /api/tags?page=1&per_page=10&search=prog
```

**Response** (200 OK):
```json
{
  "tags": [
    {
      "id": 1,
      "name": "Programming",
      "slug": "programming", 
      "description": "Notes about programming concepts",
      "usage_count": 15,
      "created_at": "2025-01-01T12:00:00Z",
      "updated_at": "2025-01-01T12:00:00Z"
    }
  ],
  "total": 25,
  "page": 1,
  "per_page": 10,
  "total_pages": 3
}
```

### 3. Get Tag Details
Retrieves detailed information for a specific tag.

**Endpoint**: `GET /api/tags/{id}`

**Response** (200 OK): Single tag object with usage statistics

### 4. Update Tag
Updates an existing tag.

**Endpoint**: `PUT /api/tags/{id}`

**Request Body**:
```json
{
  "name": "Updated Tag Name",
  "description": "Updated description"
}
```

**Response** (200 OK): Updated tag object

### 5. Delete Tag
Soft deletes a tag record.

**Endpoint**: `DELETE /api/tags/{id}`

**Response** (204 No Content): Empty body

### 6. Get Popular Tags
Retrieves tags ordered by usage count.

**Endpoint**: `GET /api/tags/popular`

**Query Parameters**:
- `limit` (integer, optional): Number of tags to return (default: 10, max: 50)

**Response** (200 OK):
```json
{
  "tags": [
    {
      "id": 1,
      "name": "Programming",
      "slug": "programming",
      "description": "Notes about programming concepts", 
      "usage_count": 25,
      "created_at": "2025-01-01T12:00:00Z"
    },
    {
      "id": 2,
      "name": "Rust",
      "slug": "rust",
      "description": "Rust programming language",
      "usage_count": 18,
      "created_at": "2025-01-01T12:00:00Z"
    }
  ]
}
```

## 📊 Response Formats

### Success Response Structure
All successful responses follow consistent patterns:

**Single Resource**:
```json
{
  "id": 1,
  "field1": "value1",
  "field2": "value2",
  // ... other fields
  "created_at": "2025-01-01T12:00:00Z",
  "updated_at": "2025-01-01T12:00:00Z"
}
```

**Collection Response**:
```json
{
  "items": [...],           // Array of resources
  "total": 100,            // Total number of items
  "page": 1,               // Current page number
  "per_page": 20,          // Items per page
  "total_pages": 5         // Total number of pages
}
```

### Timestamp Format
All timestamps use ISO 8601 format with UTC timezone:
```
"created_at": "2025-01-01T12:00:00Z"
"updated_at": "2025-01-01T12:00:00Z"
```

## ❌ Error Handling

### Error Response Format
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid request data",
    "details": {
      "field": "title",
      "message": "Title is required"
    }
  }
}
```

### Common Error Codes
- `VALIDATION_ERROR` - Request validation failed
- `NOT_FOUND` - Resource not found  
- `DUPLICATE_RESOURCE` - Resource already exists
- `INTERNAL_ERROR` - Server error

### Field Validation Errors
- **Books**: Title and author are required
- **Notes**: Content is required, note_type must be valid enum
- **Tags**: Name is required, must be unique

## 📄 Pagination

### Pagination Parameters
- `page`: Page number (starts from 1)
- `per_page`: Items per page (default: 20, max: 100)

### Pagination Response
```json
{
  "total": 150,           // Total items available
  "page": 2,              // Current page
  "per_page": 20,         // Items per page  
  "total_pages": 8        // Total pages available
}
```

### Navigation Examples
```
First page:     GET /api/books?page=1&per_page=20
Next page:      GET /api/books?page=2&per_page=20
Large page:     GET /api/books?page=1&per_page=50
```

## 🔍 Search and Filtering

### Search Capabilities
- **Books**: Search in title and author fields
- **Notes**: Search in title and content fields  
- **Tags**: Search in name field

### Filtering Options
- **Notes**: Filter by `note_type`, `book_id`, `is_favorite`
- **Tags**: Filter by usage count ranges

### Example Search Queries
```
Search books:     GET /api/books?search=rust programming
Search notes:     GET /api/notes?search=ownership&note_type=summary
Search tags:      GET /api/tags?search=prog
```

---

**API Version**: v1.0  
**Next Review**: 2025-07-25  
**Interactive Docs**: http://localhost:8080/docs