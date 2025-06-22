# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust binary application (not a library despite the name "MyLibrary"). Currently, it's a minimal "Hello, world!" program that serves as a starting point for development.

## Essential Commands

### Build and Run
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release version
- `cargo run` - Build and run the application
- `cargo run --release` - Build and run optimized version

### Code Quality
- `cargo fmt` - Format code according to Rust style guidelines
- `cargo clippy` - Run the Rust linter for code improvements
- `cargo check` - Fast type-checking without building

### Testing
- `cargo test` - Run all tests (no tests exist currently)
- `cargo test -- --nocapture` - Run tests with output displayed

## Project Structure

The codebase follows standard Rust conventions:
- `src/main.rs` - Entry point for the binary application
- `Cargo.toml` - Project manifest and dependencies
- `target/` - Build artifacts (gitignored)

## Architecture Notes

This is currently a single-file application with no modules or complex architecture. The entire application logic resides in `src/main.rs` with a simple `main()` function.

If this project evolves into a library, consider:
- Renaming `main.rs` to `lib.rs` for a pure library
- Using both `lib.rs` and `main.rs` for a library with example binary
- Creating modules in `src/` as the codebase grows