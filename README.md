[![Crates.io](https://img.shields.io/crates/v/changelog-gen.svg)](https://crates.io/crates/changelog-gen)
[![Docs.rs](https://docs.rs/changelog-gen/badge.svg)](https://docs.rs/changelog-gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/SaumilP/changelog-gen/workflows/Continuous%20Integration/badge.svg)](https://github.com/SaumilP/changelog-gen/actions)
[![codecov](https://codecov.io/gh/SaumilP/changelog-gen/branch/main/graph/badge.svg)](https://codecov.io/gh/SaumilP/changelog-gen)

# 🚀 changelog-gen

A production-grade Rust CLI tool to generate beautiful, semantic changelogs from Git history with support for conventional commits, GitHub integration, and plugin extensibility.

## ✨ Features

- 🎯 **Conventional Commits Support** - Parse and organize commits using the conventional commits standard
- 🔗 **GitHub Integration** - Automatic GitHub compare links and PR information
- 📦 **Semantic Versioning** - Built-in semantic version bumping and validation
- 📝 **Template Support** - Customizable Handlebars templates for changelog rendering
- 🔌 **Plugin System** - Extensible architecture for custom plugins
- ⚙️ **Configuration** - Support for TOML, YAML, and JSON configuration files
- 🎨 **Beautiful Output** - Colored, formatted terminal output
- 🔍 **Type-Safe** - Comprehensive error handling with meaningful error messages
- 🚦 **Enterprise-Ready** - CI/CD pipelines, security audits, code coverage
- 🌍 **Multi-Platform** - Tested on Linux, macOS, and Windows

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## 📦 Installation

### From crates.io

```bash
cargo install changelog-gen
```

### From Source

```bash
git clone https://github.com/SaumilP/changelog-gen.git
cd changelog-gen
cargo install --path .
```

### From GitHub Releases

Download pre-built binaries for your platform from [Releases](https://github.com/SaumilP/changelog-gen/releases)

## 🚀 Quick Start

```bash
# Initialize changelog configuration
changelog-gen init

# Generate changelog for current version
changelog-gen generate --release 1.0.0

# Generate with GitHub integration
changelog-gen generate --release 1.0.0 --github

# Bump version and generate changelog
changelog-gen release --bump minor
```

## 📖 Usage

### Commands

#### `init`

Initialize a new changelog configuration in the current directory.

```bash
changelog-gen init
```

Creates a default `changelog.toml` configuration file.

#### `generate`

Generate a changelog from Git history.

```bash
changelog-gen generate [OPTIONS]

OPTIONS:
    --release <VERSION>    Version for the release (e.g., 1.0.0)
    --github              Enable GitHub integration
    --conventional        Use conventional commits parsing
    --template <PATH>     Path to custom Handlebars template
    --output <PATH>       Output file path (default: CHANGELOG.md)
```

Examples:

```bash
# Generate changelog for v2.0.0
changelog-gen generate --release 2.0.0

# Generate with GitHub compare links
changelog-gen generate --release 2.0.0 --github

# Generate using conventional commits
changelog-gen generate --release 2.0.0 --conventional

# Use custom template
changelog-gen generate --release 2.0.0 --template ./my-template.hbs
```

#### `release`

Automatically bump version and generate changelog.

```bash
changelog-gen release [OPTIONS]

OPTIONS:
    --bump <TYPE>    Version bump type: major, minor, or patch
```

Examples:

```bash
# Bump patch version
changelog-gen release --bump patch

# Bump minor version
changelog-gen release --bump minor

# Bump major version (breaking changes)
changelog-gen release --bump major
```

## ⚙️ Configuration

Configuration files can be placed in the project root with any of these names:
- `changelog.toml`
- `changelog.yaml`
- `changelog.yml`

### Configuration Example

```toml
[project]
name = "my-project"
repository = "https://github.com/user/my-project"

[notifications]
slack_webhook = "https://hooks.slack.com/services/..."
discord_webhook = "https://discordapp.com/api/webhooks/..."

[telemetry]
enabled = true
```

## 🧪 Testing

Run the test suite:

```bash
cargo test --all
```

## 🔒 Security

See [SECURITY.md](SECURITY.md) for security policies and responsible disclosure.

## 📚 Documentation

- [Full API Documentation](https://docs.rs/changelog-gen)
- [Contributing Guide](CONTRIBUTING.md)
- [Versioning Strategy](VERSIONING.md)
- [Changelog](CHANGELOG.md)

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
# Clone repository
git clone https://github.com/SaumilP/changelog-gen.git
cd changelog-gen

# Build
cargo build

# Run tests
cargo test --all

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🎉 Acknowledgments

Built with:
- [Tokio](https://tokio.rs/) - Async runtime
- [Clap](https://github.com/clap-rs/clap) - CLI parsing
- [Git2](https://github.com/rust-lang/git2-rs) - Git operations
- [Serde](https://serde.rs/) - Serialization
- [Handlebars](https://handlebarsjs.com/) - Templates

## 📞 Support

- [GitHub Issues](https://github.com/SaumilP/changelog-gen/issues)
- [Discussions](https://github.com/SaumilP/changelog-gen/discussions)
- [Documentation](https://docs.rs/changelog-gen)

---

## 🗺️ Roadmap

- [ ] v0.2.0: Enhanced GitHub workflow templates
- [ ] v0.3.0: Docker support and container publishing
- [ ] v1.0.0: Stable API release
- [ ] v1.1.0: Performance benchmarking suite

**Made with ❤️ by [Saumil Patel](https://github.com/SaumilP)**
