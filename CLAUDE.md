# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal reading notes management system backend implemented in Rust. The application provides RESTful APIs for managing books, reading notes, categories, tags, and reading progress. Built with Actix-web and PostgreSQL.

## Essential Commands

### Build and Run
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release version
- `cargo run` - Build and run the application
- `cargo run --release` - Build and run optimized version

### Database
- `diesel migration run` - Run pending database migrations
- `diesel migration revert` - Revert the last migration
- `diesel setup` - Set up the database and run migrations
- `docker-compose up -d` - Start PostgreSQL container

### Code Quality
- `cargo fmt` - Format code according to Rust style guidelines
- `cargo clippy` - Run the Rust linter for code improvements
- `cargo check` - Fast type-checking without building

### Testing
- `cargo test` - Run all tests
- `cargo test -- --nocapture` - Run tests with output displayed
- `cargo test --test integration_test` - Run specific integration tests
- `cargo test --test database_test` - Run database tests

## Project Structure

The codebase follows modular Rust architecture:
- `src/main.rs` - Entry point for the binary application
- `src/lib.rs` - Library entry point with app factory
- `src/handlers/` - HTTP request handlers for each resource
- `src/models/` - Database models and structs
- `src/db/` - Database connection and schema
- `src/config/` - Configuration management
- `src/middleware/` - Custom middleware
- `src/errors/` - Error handling
- `src/utils/` - Utility functions
- `tests/` - Integration and database tests
- `migrations/` - Database migration files
- `docs/` - Project documentation

## Architecture Notes

This is a full-featured web API backend with:
- Actix-web for HTTP server and routing
- Diesel ORM for PostgreSQL database operations
- R2D2 connection pooling
- Structured error handling
- CORS support for frontend integration
- Soft delete pattern for data integrity
- Comprehensive test coverage

## Database Design

The system uses PostgreSQL with the following main entities:
- Books (with ISBN, title, authors, etc.)
- Categories (hierarchical organization)
- Tags (flexible labeling system)
- Reading Status (progress tracking)
- Reading Notes (markdown support)
- Association tables for many-to-many relationships

All tables implement soft delete using `deleted_at` timestamps and use BIGINT for primary keys optimized for PostgreSQL.