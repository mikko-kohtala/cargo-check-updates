# CLAUDE.md

AI Assistant Context for cargo-check-updates (ccu)

## Project Overview

**cargo-check-updates** is a Rust CLI tool that upgrades Cargo.toml dependencies to their latest versions, inspired by npm-check-updates.

Binary names: `ccu` and `cargo-check-updates`

## Purpose

Unlike `cargo update` (which only updates Cargo.lock within semver constraints), this tool:
- Updates version specifications in Cargo.toml itself
- Can upgrade beyond existing semver ranges
- Provides interactive mode for selective upgrades
- Includes doctor mode to identify breaking changes

## Project Structure

```
src/
├── main.rs           # Entry point, CLI parsing with clap
├── lib.rs            # Library exports
├── cli.rs            # CLI definitions (clap structs)
├── commands/         # Command implementations
│   ├── mod.rs
│   └── check.rs      # Check and upgrade logic
├── cargo/            # Cargo.toml operations
│   ├── mod.rs
│   └── parser.rs     # Parse/modify Cargo.toml (toml_edit)
├── registry/         # crates.io interaction
│   ├── mod.rs
│   └── client.rs     # HTTP client (reqwest + tokio)
├── ui/               # User interface
│   ├── mod.rs
│   └── output.rs     # Colored output formatting
└── error.rs          # Error types (thiserror)
```

## Key Dependencies

- **clap 4.5+**: CLI argument parsing
- **tokio 1.36+**: Async runtime for parallel registry queries
- **reqwest 0.12+**: HTTP client for crates.io API
- **toml_edit 0.22+**: Parse/modify TOML preserving formatting
- **semver 1.0**: Version comparison
- **colored 2.1**: Terminal colors
- **anyhow/thiserror**: Error handling

## Development Commands

```bash
make build      # Debug build
make release    # Release build
make check      # Quick compilation check
make test       # Run tests
make fmt        # Format code
make clippy     # Lint with clippy
make install    # Install locally (both binaries)
make dev        # Format + lint + test
make ci         # CI workflow
```

## Technical Decisions

- **toml_edit**: Preserves formatting when modifying Cargo.toml
- **Async**: Use tokio for parallel crates.io queries
- **Non-destructive**: Default mode only shows updates, `-u` flag required to modify

## Coding Conventions

- Use `anyhow::Result` for application errors
- Use `thiserror` for library error types
- Async functions for I/O operations
- Comprehensive error messages for user-facing errors

## Current Status

Foundation is complete with stub implementations. Actual logic to be implemented:
- Cargo.toml parsing (extract dependencies)
- crates.io API integration (query versions)
- Version comparison and update logic
- Interactive mode
- Doctor mode with testing
