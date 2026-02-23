# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-17

### Added
- Initial release of changeloggen-cli
- Comprehensive error handling system with thiserror integration
- Conventional Commits support for commit grouping
- Git repository commit parsing
- Handlebars template support for changelog rendering
- Command-line interface with multiple commands (init, generate, release)
- Configuration file support (TOML, YAML, JSON)
- Telemetry and tracing capabilities
- Workspace detection for monorepo support
- GitHub API integration for pull request handling
- Plugin system with dynamic loading
- Semantic versioning utilities
- Complete test coverage
- Production-ready CI/CD pipelines (GitHub Actions)

### Features
- **Multi-platform Support**: Tested on Linux, macOS, and Windows
- **Async/await**: Built on Tokio for async operations
- **Type-safe**: Comprehensive error types using thiserror
- **Configurable**: TOML, YAML support for customization
- **Extensible**: Plugin system for custom functionality
- **Enterprise-ready**: Security audits, code coverage, versioning

### Documentation
- Comprehensive README with examples
- VERSIONING.md for semantic versioning strategy
- CONTRIBUTING.md for contribution guidelines
- Error handling documentation

### Infrastructure
- GitHub Actions CI/CD pipeline with multi-platform builds
- Automated release process to crates.io
- Code coverage reporting
- Security vulnerability scanning
- Pre-release validation

## [Unreleased]

### Planned
- Enhanced GitHub workflow templates
- Docker image publication
- NPM package mirror
- Performance benchmarking suite
