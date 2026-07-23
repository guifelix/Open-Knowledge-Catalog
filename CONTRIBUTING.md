# Contributing to Open Knowledge Catalog

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Ways to Contribute

- **Bug Reports**: Found an issue? [Open an issue](https://github.com/your-org/open-knowledge-catalog/issues)
- **Feature Requests**: Have an idea? [Start a discussion](https://github.com/your-org/open-knowledge-catalog/discussions)
- **Code Contributions**: Fix bugs, add features, improve documentation
- **Documentation**: Improve README, add examples, write tutorials
- **Testing**: Add test cases, improve test coverage

## Development Setup

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- SQLite 3.38+ (usually pre-installed on Linux/macOS)
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/your-org/open-knowledge-catalog.git
cd open-knowledge-catalog

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

## Code Style

We follow standard Rust conventions:

- Format with `cargo fmt`
- Lint with `cargo clippy -- -D warnings`
- Run tests with `cargo test`

Before submitting a PR, ensure:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Make** your changes with tests
4. **Run** the full test suite locally
5. **Submit** a pull request with a clear description

### PR Requirements

- Clear, descriptive title
- Link to related issue (if applicable)
- Tests for new functionality
- Updated documentation (README, doc comments)
- No clippy warnings
- Formatted code

## Adding New Features

### AI Tool Operations

When adding a new AI-facing operation:

1. Add the operation to `RepositoryIndex` in `src/index/database.rs`
2. Expose it in `OkfService` in `src/service/mod.rs`
3. Add CLI command in `src/transport/cli.rs`
4. Add integration tests with fixture data
5. Update the AI usage examples in README

### Parser Changes

When modifying parsers:

1. Add unit tests in the relevant module's `#[cfg(test)]` section
2. Add property-based tests with `proptest` for edge cases
3. Test with the YAML test suite and CommonMark spec examples
4. Update the test fixture repository if needed

## Test Guidelines

### Unit Tests

- Test individual parser functions with various inputs
- Test error conditions and edge cases
- Use `proptest` for property-based testing of parsing logic

### Integration Tests

- Create fixture repositories in `tests/fixtures/`
- Test full scan → index → query workflows
- Validate AI tool responses match expected patterns

### Example Test Fixture

```markdown
# tests/fixtures/simple/metrics/revenue.md
---
type: Metric
title: Revenue
tags: [finance]
---

# Definition
Revenue is income.
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments (`///`) for public APIs
- Include examples in doc comments where helpful
- Update AI usage examples for new operations

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a git tag
4. GitHub Actions will build and publish

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, you agree to uphold this code.

## Questions?

- Open a [discussion](https://github.com/your-org/open-knowledge-catalog/discussions)
- Check existing [issues](https://github.com/your-org/open-knowledge-catalog/issues)
- Review the [architecture docs](doc1.md) and [library analysis](doc2.md)