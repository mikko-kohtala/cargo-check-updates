.PHONY: all build check compile test fmt fmt-check clippy clippy-all install clean doc dev ci run release update audit help

# Default target
all: check

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run clippy and tests
check: clippy test

# Quick compilation check (faster than build)
compile:
	cargo check --all-targets

# Run tests
test:
	cargo test --verbose

# Run tests with output
test-output:
	cargo test -- --nocapture

# Format code
fmt:
	cargo fmt --all

# Check formatting without changing files
fmt-check:
	cargo fmt --all -- --check

# Run clippy linter
clippy:
	cargo clippy --all-targets -- -D warnings

# Run clippy with all features
clippy-all:
	cargo clippy --all-targets --all-features -- -D warnings

# Install locally (installs both 'ccu' and 'cargo-check-updates' binaries)
install:
	cargo install --path .

# Clean build artifacts
clean:
	cargo clean

# Generate and open documentation
doc:
	cargo doc --no-deps --open

# Development workflow (format, lint, test)
dev: fmt clippy test

# CI workflow (check format, lint, test)
ci: fmt-check clippy test

# Run the binary directly (as 'ccu')
run:
	cargo run --bin ccu --

# Run with example (check current directory)
example:
	cargo run --bin ccu -- --manifest-path Cargo.toml

# Update dependencies
update:
	cargo update

# Audit dependencies for security vulnerabilities (requires cargo-audit)
audit:
	cargo audit

# Show help
help:
	@echo "Available targets:"
	@echo "  make          - Run check (default)"
	@echo "  make check    - Run clippy and tests"
	@echo "  make compile  - Quick compilation check"
	@echo "  make build    - Build in debug mode"
	@echo "  make release  - Build in release mode"
	@echo "  make test     - Run tests"
	@echo "  make fmt      - Format code"
	@echo "  make clippy   - Run linter"
	@echo "  make install  - Install binaries (ccu and cargo-check-updates)"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make doc      - Generate and open documentation"
	@echo "  make dev      - Development workflow (fmt + clippy + test)"
	@echo "  make ci       - CI workflow (fmt-check + clippy + test)"
	@echo "  make run      - Run the binary"
	@echo "  make help     - Show this help message"
