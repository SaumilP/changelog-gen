# Contributing to changelog-gen

Thank you for your interest in contributing to changelog-gen! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps which reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed**
* **Explain which behavior you expected to see instead**
* **Include screenshots if possible**
* **Include your environment details** (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a step-by-step description of the suggested enhancement**
* **Provide specific examples to demonstrate the steps**
* **Describe the current behavior and expected behavior**
* **Explain why this enhancement would be useful**

### Pull Requests

* Fill in the required template
* Follow the Rust styleguide
* Include appropriate test cases
* Update documentation as needed
* Ensure all tests pass

## Development Setup

### Prerequisites

* Rust 1.70+ (MSRV: 1.70)
* Git
* Cargo

### Setup Local Environment

```bash
# Clone the repository
git clone https://github.com/SaumilP/changelog-gen.git
cd changelog-gen

# Install dependencies and build
cargo build

# Run tests
cargo test --all

# Run with logging
RUST_LOG=debug cargo run -- --help
```

### Development Workflow

1. **Create a feature branch**
   ```bash
   git checkout -b feature/my-feature develop
   ```

2. **Make your changes**
   ```bash
   # Edit files, add tests
   ```

3. **Run tests and checks**
   ```bash
   cargo test --all
   cargo fmt
   cargo clippy
   ```

4. **Commit your changes**
   ```bash
   git commit -m "feat: description of change"
   ```

5. **Push and create a Pull Request**
   ```bash
   git push origin feature/my-feature
   ```

## Code Style

This project follows standard Rust conventions:

* Use `cargo fmt` for formatting
* Use `cargo clippy` for linting
* Write descriptive commit messages
* Add tests for new functionality
* Document public APIs with doc comments

### Commit Message Format

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(parser): add support for angular commits

Allow parser to handle Angular style commits with scope.
Fixes #123
```

## Testing

* Write tests for new features
* Ensure all existing tests pass
* Run full test suite before submitting PR
* Aim for >80% code coverage

```bash
# Run all tests
cargo test --all

# Run with verbose output
cargo test --all -- --nocapture

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin
```

## Documentation

* Update README.md if changing user-facing features
* Add doc comments to public APIs
* Update CHANGELOG.md with user-facing changes
* Include examples in documentation

## Versioning

This project follows [Semantic Versioning](https://semver.org/). See [VERSIONING.md](VERSIONING.md) for details.

## Build and Release Process

### For Maintainers

Releases are automated via GitHub Actions. To release:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes with message `chore: release v1.2.3`
4. Tag commit: `git tag v1.2.3`
5. Push tags: `git push origin main --tags`

GitHub Actions will:
- Run full test suite
- Build binaries for multiple platforms
- Create GitHub release
- Publish to crates.io

## Questions?

Feel free to open an issue with the `question` label.

## Additional Notes

* This is a volunteer-run project
* Respect everyone's time and effort
* Be constructive and professional
* Thank you for contributing!
