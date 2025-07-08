# Makefile for eddi-pad
# Rust project with standardized targets

# Default target
.PHONY: all
all: build

# Build targets
.PHONY: build
build:
	@echo "Building eddi-pad..."
	@cargo build

.PHONY: build-release
build-release:
	@echo "Building eddi-pad in release mode..."
	@cargo build --release

# Test targets
.PHONY: test
test:
	@echo "Running tests..."
	@cargo test

.PHONY: test-watch
test-watch:
	@echo "Running tests in watch mode..."
	@cargo watch -x test

# Code quality targets
.PHONY: lint
lint:
	@echo "Checking code quality..."
	@cargo clippy -- -D warnings
	@cargo fmt --check

.PHONY: format
format:
	@echo "Formatting code..."
	@cargo fmt

.PHONY: lint-check
lint-check: lint

# Git operations with CI monitoring
.PHONY: push
push:
	@echo "Pushing changes with workflow monitoring..."
	@git push && gh run list || echo "No workflows found or gh not available"

.PHONY: workflow-status
workflow-status:
	@echo "Checking workflow status..."
	@gh run list --limit 5 || echo "No workflows found or gh not available"

.PHONY: watch-workflows
watch-workflows:
	@echo "Watching workflows..."
	@gh run watch || echo "No active workflows or gh not available"

# Git status
.PHONY: status
status:
	@echo "Git status:"
	@git status

# Development targets
.PHONY: setup
setup:
	@echo "Setting up development environment..."
	@cargo --version
	@echo "✅ Cargo is available"
	@rustc --version
	@echo "✅ Rust is available"

.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

.PHONY: run
run:
	@echo "Running eddi-pad..."
	@cargo run

.PHONY: run-release
run-release:
	@echo "Running eddi-pad in release mode..."
	@cargo run --release

# Help target
.PHONY: help
help:
	@echo "eddi-pad Makefile targets:"
	@echo "  build            - Build the project"
	@echo "  build-release    - Build in release mode"
	@echo "  test             - Run tests"
	@echo "  test-watch       - Run tests in watch mode"
	@echo "  lint             - Check code quality (clippy + fmt check)"
	@echo "  format           - Format code with rustfmt"
	@echo "  lint-check       - Same as lint (compatibility)"
	@echo "  push             - Push changes with workflow monitoring"
	@echo "  workflow-status  - Check CI workflow status"
	@echo "  watch-workflows  - Watch active workflows"
	@echo "  status           - Show git status"
	@echo "  setup            - Setup development environment"
	@echo "  clean            - Clean build artifacts"
	@echo "  run              - Run the application"
	@echo "  run-release      - Run in release mode"
	@echo "  help             - Show this help message"