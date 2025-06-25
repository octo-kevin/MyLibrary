# System Requirements

> **Document Info**  
> Version: v2.0  
> Last Updated: 2025-06-25  
> Author: Development Team  
> Status: Final

## 📋 Table of Contents

- [Project Overview](#project-overview)
- [Functional Requirements](#functional-requirements)
- [Technical Architecture](#technical-architecture)
- [Data Model](#data-model)
- [API Interface Design](#api-interface-design)
- [User Interface Design](#user-interface-design)
- [Development Plan](#development-plan)
- [Technical Implementation](#technical-implementation)
- [Optional Extensions](#optional-extensions)

## 🎯 Project Overview

Personal Reading Notes Management System - A comprehensive full-stack application for tracking personal reading history and managing reading notes. The system supports book management, reading status tracking, note-taking, and provides search and statistical features.

### Core Features
- 📚 Book information management
- 📝 Reading notes with Markdown support
- 🔍 Advanced search functionality
- 📊 Reading statistics and analytics
- 🏷️ Flexible tagging system

## 📋 Functional Requirements

### 1. Book Management

#### Book Operations
- **Add Books**
  - Manual book information entry (title, author, category, pages, etc.)
  - Optional ISBN input
  - Reading status setting (want-to-read, reading, completed)
  
- **Book Information Management**
  - Modify basic book information
  - Update reading status
  - Record reading time (start date, completion date)
  - Rate books (1-5 star rating)
  - Delete book records (soft delete)

- **Book Categorization**
  - Custom category support (e.g., Technology, Literature, History)
  - Browse books by category
  - Hierarchical category structure

### 2. Reading Notes

#### Note Creation and Management
- **Note Writing**
  - Create multiple notes per book
  - Markdown format support
  - Note type classification (Quote, Summary, Thought, General)
  - Page reference support

- **Note Management**
  - Edit and delete notes
  - Search note titles and content
  - Sort notes by time
  - Tag association

### 3. Search Functionality

#### Book Search
- Search by book title
- Search by author
- Filter by category
- Filter by reading status
- Filter by rating

#### Note Search
- Full-text search across all notes
- Search within specific book notes
- Search by note type
- Search by tags

### 4. Statistics and Analytics

#### Reading Statistics
- Annual/monthly reading quantity statistics
- Reading distribution by category
- Average reading speed (based on pages and time)
- Rating distribution statistics

#### Reading Timeline
- Chronological reading history display
- Visual reading progress representation

## 🛠️ Technical Architecture

### Technology Stack
```
Frontend: React 19 + TypeScript 5.8 + Vite 7.0 + Ant Design 5.26.2
Backend:  Rust + Actix-web 4.9 + PostgreSQL 17.5 + Diesel ORM 2.2
Database: PostgreSQL with full-text search
Build:    pnpm + Cargo
Docs:     OpenAPI 3.0 + Swagger UI
Testing:  70+ automated tests
```

### System Architecture
```
┌─────────────────────┐
│   React Frontend    │
│  (Ant Design UI)    │
└──────────┬──────────┘
           │ HTTP REST API
┌──────────▼──────────┐
│   Rust Backend      │
│   (Actix-web)       │
└──────────┬──────────┘
           │ SQL + Diesel ORM
┌──────────▼──────────┐
│    PostgreSQL       │
│     Database        │
└─────────────────────┘
```

## 🗄️ Data Model

### Design Philosophy
- **Table Structure Separation**: Frequently updated attributes (reading status, ratings) in separate tables
- **Soft Delete Mechanism**: All tables support soft delete to preserve historical data
- **Flexible Tagging System**: Support both categories and tags for classification
- **Performance Optimization**: Partial indexes for non-deleted data only

### Core Data Tables

For detailed database design, see: [Database Design Document](../design/database/database-design.md)

Main tables include:
1. **books** - Basic book information (rarely updated)
2. **reading_notes** - Reading notes table (4 types, page references)
3. **tags** - Tag system (auto-generated slugs, usage tracking)
4. **note_tag_associations** - Note-tag many-to-many relationships

### Soft Delete Implementation
All tables include `deleted_at` field for soft delete:
- Delete: set `deleted_at = CURRENT_TIMESTAMP`
- Query: add condition `WHERE deleted_at IS NULL`
- Support data recovery

## 🔗 API Interface Design

### Books Management API

#### 1. Get Books List
- **Endpoint**: `GET /api/books`
- **Query Parameters**:
  - `page`: Page number (default 1)
  - `per_page`: Items per page (default 20)
  - `search`: Search keyword
- **Response**:
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
        "created_at": "2025-01-01T00:00:00Z"
      }
    ],
    "total": 50,
    "page": 1,
    "per_page": 20,
    "total_pages": 3
  }
  ```

#### 2. Get Book Details
- **Endpoint**: `GET /api/books/{id}`
- **Response**: Complete book information with related notes

#### 3. Create Book
- **Endpoint**: `POST /api/books`
- **Request Body**:
  ```json
  {
    "title": "Book Title",
    "author": "Author Name",
    "isbn": "978-xxx-xxx",
    "publisher": "Publisher",
    "page_count": 300,
    "description": "Book description"
  }
  ```

#### 4. Update Book
- **Endpoint**: `PUT /api/books/{id}`
- **Request Body**: Fields to update

#### 5. Delete Book
- **Endpoint**: `DELETE /api/books/{id}`

#### 6. Get Book Notes
- **Endpoint**: `GET /api/books/{id}/notes`

### Reading Notes API

#### 1. Get Notes List
- **Endpoint**: `GET /api/notes`
- **Query Parameters**:
  - `page`: Page number
  - `per_page`: Items per page
  - `search`: Search keyword
  - `note_type`: Filter by note type
- **Response**:
  ```json
  {
    "notes": [
      {
        "id": 1,
        "title": "Chapter 1 Notes",
        "content": "Reading note content...",
        "note_type": "summary",
        "page_reference": 15,
        "is_favorite": false,
        "tags": ["programming", "rust"],
        "book": {
          "id": 1,
          "title": "The Rust Programming Language"
        },
        "created_at": "2025-01-02T10:00:00Z"
      }
    ],
    "total": 120,
    "page": 1,
    "per_page": 20,
    "total_pages": 6
  }
  ```

#### 2. Create Note
- **Endpoint**: `POST /api/notes`
- **Request Body**:
  ```json
  {
    "title": "Note Title",
    "content": "Note content (Markdown supported)",
    "note_type": "summary",
    "book_id": 1,
    "page_reference": 15,
    "is_favorite": false,
    "tags": ["programming", "rust"]
  }
  ```

#### 3. Update Note
- **Endpoint**: `PUT /api/notes/{id}`

#### 4. Delete Note
- **Endpoint**: `DELETE /api/notes/{id}`

#### 5. Update Note Tags
- **Endpoint**: `PUT /api/notes/{id}/tags`

### Tags API

#### 1. Get Tags List
- **Endpoint**: `GET /api/tags`
- **Query Parameters**:
  - `page`: Page number
  - `per_page`: Items per page
  - `search`: Search keyword

#### 2. Get Popular Tags
- **Endpoint**: `GET /api/tags/popular`

#### 3. Create Tag
- **Endpoint**: `POST /api/tags`

#### 4. Update Tag
- **Endpoint**: `PUT /api/tags/{id}`

#### 5. Delete Tag
- **Endpoint**: `DELETE /api/tags/{id}`

## 🎨 User Interface Design

### 1. Main Dashboard
- Reading statistics overview
- Recently read books
- Quick add book entry
- Reading progress indicators

### 2. Books List Page
- Book grid/list view
- Category filter sidebar
- Search functionality
- Status filter tags
- Pagination controls

### 3. Book Details Page
- Complete book information display
- Rating component
- Reading status updates
- All book notes list
- Add note button

### 4. Note Editor Page
- Markdown editor with live preview
- Note type selection
- Page reference input
- Tag management
- Save/cancel actions

### 5. Statistics Page
- Reading quantity charts
- Category distribution pie charts
- Reading timeline
- Annual reading goals progress

## 📅 Development Plan

### Phase 1: Core Functionality (2 weeks) ✅
1. Project initialization and database design
2. Books CRUD API implementation
3. Basic frontend pages (books list, details, add)

### Phase 2: Notes Functionality (1 week) ✅
1. Reading notes CRUD API
2. Markdown editor integration
3. Notes management interface

### Phase 3: Search and Statistics (1 week) ✅
1. Search functionality implementation
2. Statistics API and pages
3. Data visualization

### Phase 4: Optimization and Polish (1 week) ✅
1. UI optimization and responsive design
2. Data import/export functionality
3. Performance optimization and testing

## 🔧 Technical Implementation

### 1. Data Storage
- PostgreSQL database with advanced features
- Data backup and recovery functionality
- Export support (JSON/CSV formats)
- Diesel ORM for type-safe database operations

### 2. Markdown Support
- Markdown syntax support in note editing
- Real-time preview using react-markdown
- Code highlighting integration
- Export notes to HTML/PDF formats

### 3. Search Functionality
- PostgreSQL full-text search capabilities
- Chinese word segmentation support
- Search result highlighting
- Fuzzy and exact search support

### 4. Performance Optimization
- Proper database indexing
- API response pagination
- React component lazy loading
- React Query for data caching
- Image lazy loading and CDN optimization

### 5. Frontend Technical Points
- TypeScript for type safety
- Component-based development for code reuse
- React Router for routing management
- Responsive design for mobile support
- PWA support for offline access

### 6. Backend Technical Points
- RESTful API design
- Request parameter validation
- Error handling and logging
- API rate limiting and security protection
- Comprehensive test coverage (70+ tests)

## 🚀 Optional Extensions

### 1. Data Synchronization
- Multi-device data sync support
- Cloud backup functionality

### 2. Social Features
- Share reading notes
- Export as blog articles

### 3. Smart Recommendations
- Book recommendations based on reading history
- Similar book suggestions

### 4. Mobile Enhancement
- Progressive Web App (PWA)
- Offline functionality
- Mobile-optimized interface

---

**Document Version**: v2.0  
**Next Review**: 2025-07-25  
**Project Status**: 95% Complete - Production Ready