# OxiCloud Development Guide

## Build Commands
```bash
# Core development workflow
cargo build                 # Build the project
cargo run                   # Run the project locally (server at http://127.0.0.1:8085)
cargo check                 # Quick check for compilation errors without building

# Testing commands
cargo test                  # Run all tests
cargo test -- --nocapture   # Run tests with output displayed
cargo test <test_name>      # Run a specific test (e.g., cargo test file_service)
cargo test domain::entities::file::tests::test_create_file  # Run a specific test function
RUST_LOG=debug cargo test   # Run tests with debug-level logging
RUST_LOG=trace cargo test   # Run tests with trace-level logging

# Code quality tools
cargo clippy                # Run linter to catch common mistakes
cargo clippy --fix          # Fix auto-fixable linting issues
cargo fmt --check           # Check code formatting without changing files
cargo fmt                   # Format code according to Rust conventions

# Debugging
RUST_LOG=debug cargo run    # Run with detailed logging for debugging
RUST_BACKTRACE=1 cargo run  # Run with full backtrace for better error diagnostics
```

## Code Style Guidelines
- **Architecture**: Follow Clean Architecture with clear layer separation (domain → application → infrastructure → interfaces)
- **Naming**: Use `snake_case` for files, modules, functions, variables; `PascalCase` for types/structs/enums; getters without `get_` prefix
- **Modules**: Use mod.rs files for explicit exports with visibility modifiers (pub, pub(crate))
- **Error Handling**: Use Result<T, E> with thiserror for custom error types; propagate errors with ? operator; include context in error messages
- **Documentation**: Document public APIs with /// doc comments, explain "why" not "what"; both English and Spanish comments are acceptable
- **Imports**: Group imports: 1) std, 2) external crates, 3) internal modules (with blank lines between)
- **Async**: Use async-trait for repository interfaces; handle futures with .await and tokio runtime; implement timeouts for I/O operations
- **Testing**: Write unit tests in the same file as implementation (bottom of file, in a tests module with #[cfg(test)])
- **Dependencies**: Use axum for web API, tower-http for middleware, serde for serialization; share dependencies with Arc
- **Logging**: Use tracing with appropriate levels (debug, info, warn, error) and structured contexts for detailed diagnostics
- **Repository Pattern**: Define interfaces in domain layer, implement in infrastructure layer; use traits with dynamic dispatch (Box<dyn Trait>)
- **I18n**: Store translations in JSON files under static/locales/, use i18n service for text lookups
- **Type Safety**: Prefer strong typing with domain-specific types over primitive types; validate at construction time
- **Error Messages**: Provide clear, actionable error messages that help diagnose the issue
- **Immutability**: Prefer immutable data structures; use with_* methods to return modified copies rather than mutating in place
- **Performance**: Implement caching with proper invalidation; use parallel processing for large file operations; optimize based on file sizes
- **Auth & Security**: Follow JWT-based authentication; hash passwords with Argon2; implement proper CSRF protection
- **File Operations**: Use appropriate strategies based on file size; implement proper concurrency for large files

## Project Structure
OxiCloud is a NextCloud-like file storage system built in Rust with a focus on performance and security. It provides a clean REST API and web interface for file management using a layered architecture approach:

- **Domain Layer**: Core business logic and entities (src/domain/)
- **Application Layer**: Use cases and application services (src/application/)
- **Infrastructure Layer**: External systems and implementations (src/infrastructure/)
- **Interfaces Layer**: API and web controllers (src/interfaces/)

The roadmap in TODO-LIST.md outlines planned features including enhanced folder support, file previews, user authentication, sharing, and a sync client.