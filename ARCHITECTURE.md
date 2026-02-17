# Architecture

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Core Modules](#core-modules)
- [Error Handling](#error-handling)
- [Data Flow](#data-flow)
- [Module Responsibilities](#module-responsibilities)
- [Design Patterns](#design-patterns)
- [CI/CD Architecture](#cicd-architecture)
- [Build & Release](#build--release)
- [Security Architecture](#security-architecture)
- [Extension Points](#extension-points)
- [Dependencies](#dependencies)

---

## Overview

changelog-gen is a production-grade CLI tool following clean architecture principles with clear separation of concerns, comprehensive error handling, and enterprise-grade CI/CD infrastructure. The architecture emphasizes:

- **Modularity**: Well-defined module boundaries with single responsibilities
- **Type Safety**: Leverages Rust's type system with custom error types
- **Testability**: Pure functions with minimal side effects
- **Extensibility**: Plugin system and trait-based abstraction
- **Maintainability**: Clear module organization and comprehensive documentation

---

## Project Structure

```
changelog-gen/
├── src/
│   ├── main.rs                 # Entry point with error handling
│   ├── lib.rs                  # Library root exports
│   ├── error.rs                # Error type definitions (16 variants)
│   ├── bootstrap.rs            # Tracing initialization
│   ├── parser.rs               # Commit parsing logic
│   │
│   ├── application/            # Application layer (commands)
│   │   ├── mod.rs              # Module exports
│   │   ├── commands.rs         # CLI command dispatch
│   │   └── services.rs         # Business logic orchestration
│   │
│   ├── cli/                    # CLI interface
│   │   ├── mod.rs              # CLI argument parser (clap)
│   │   ├── generate.rs         # Generate command
│   │   ├── init.rs             # Initialize command
│   │   ├── release.rs          # Release command
│   │   └── self_update.rs      # Update command
│   │
│   ├── domain/                 # Domain models (clean architecture)
│   │   ├── mod.rs              # Module exports
│   │   ├── changelog.rs        # Changelog generation logic
│   │   ├── commit.rs           # Commit struct (Commit data model)
│   │   └── version.rs          # Version utilities
│   │
│   ├── infrastructure/         # Infrastructure & adapters
│   │   ├── mod.rs              # Module exports
│   │   ├── git.rs              # Git2 adapter implementation
│   │   ├── github.rs           # GitHub API client
│   │   ├── plugins.rs          # Plugin system
│   │   ├── template.rs         # Handlebars template adapter
│   │   ├── templates.rs        # Template utilities
│   │   └── workspace.rs        # Workspace detection
│   │
│   ├── traits/                 # Trait abstractions
│   │   ├── mod.rs              # Module exports
│   │   ├── git.rs              # GitRepository trait
│   │   └── template.rs         # TemplateRenderer trait
│   │
│   ├── config/                 # Configuration management
│   │   ├── mod.rs              # Module exports
│   │   ├── loader.rs           # Config file loader
│   │   └── schema.rs           # Config data structures
│   │
│   ├── services/               # Service layer
│   │   ├── generator.rs        # Changelog generation service
│   │   ├── notifier.rs         # Notification service
│   │   ├── release_service.rs  # Release service
│   │   └── telemetry.rs        # Telemetry/analytics
│   │
│   ├── plugins/                # Plugin interface
│   │   └── mod.rs              # Plugin trait definitions
│   │
│   └── app.rs                  # Application state (future)
│
├── tests/                      # Integration tests
│   ├── changelog_tests.rs      # Changelog generation tests
│   ├── config_tests.rs         # Configuration tests
│   ├── parser_tests.rs         # Parser tests
│   └── version_tests.rs        # Version utility tests
│
├── templates/                  # Handlebars templates
│   └── default.hbs             # Default changelog template
│
├── .github/workflows/          # GitHub Actions CI/CD
│   ├── ci.yml                  # Continuous Integration pipeline
│   └── release.yml             # Release & publishing pipeline
│
├── Cargo.toml                  # Project manifest & dependencies
├── CHANGELOG.md                # Version history
├── VERSIONING.md               # Semantic versioning strategy
├── CONTRIBUTING.md             # Contribution guidelines
├── SECURITY.md                 # Security policy
├── PUBLISHING.md               # Publishing guide
├── LICENSE                     # MIT License
└── README.md                   # User documentation
```

---

## Core Modules

### 1. **Error Module** (`src/error.rs`)

Comprehensive error handling using `thiserror` crate.

**16 Error Variants**:
```rust
pub enum ChangelogError {
    GitError(String),                    // Git operations failed
    RepositoryNotFound(PathBuf),        // Repo not found
    CommitParseError(String),           // Commit message parsing
    VersionParseError(String),          // Version parsing
    ConfigNotFound(PathBuf),            // Configuration file
    ConfigParseError { format, reason }, // Config parsing
    TemplateError(String),              // Template rendering
    PluginLoadError(String),            // Plugin loading
    GitHubApiError(String),             // GitHub API
    NetworkError(String),               // Network operations
    IoError(#[from] std::io::Error),   // File I/O
    InvalidArguments(String),           // CLI arguments
    TemplateNotFound(PathBuf),          // Template file
    OutputWriteError(PathBuf, String), // Output file
    RegexError(String),                // Regex compilation
    WorkspaceNotFound(String),         // Workspace detection
    TelemetryError(String),            // Analytics
    UnsupportedFeature(String),        // Feature not available
    Other(String),                     // Generic error
}
```

**Exit Codes** (1-99 range):
- 1: Invalid arguments
- 2: Git errors
- 3: Configuration errors
- 4-5: Template/versioning errors
- 6-7: Network/I/O errors
- 99: Other errors

**Helper Methods**:
- `exit_code()` - Get appropriate exit code
- `is_recoverable()` - Determine if error is recoverable
- `Result<T>` - Convenience type alias

### 2. **CLI Layer** (`src/cli/`)

Command-line interface using `clap` derive macros.

**Command Structure**:
```rust
pub enum Commands {
    Init,                           // Initialize config
    Generate { release, github },   // Generate changelog
    Release { bump },               // Bump version & release
}
```

**Features**:
- Subcommand-based interface
- Type-safe argument parsing
- Auto-generated help text
- Validation at parse time

### 3. **Domain Layer** (`src/domain/`)

Pure domain logic with no side effects.

**Commit Model**:
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit {
    pub message: String,  // Commit message
    pub hash: String,     // Commit hash
}
```

**Changelog Generation**:
```rust
pub fn generate(
    grouped: GroupedCommits,    // Organized commits
    release: Option<String>,    // Version number
    github_enabled: bool,       // GitHub integration
) -> Result<String>
```

**Version Utilities**:
- Semantic version parsing
- Version bumping (major/minor/patch)
- Validation

### 4. **Parser Module** (`src/parser.rs`)

Commit message parsing and organization.

**Parsing Strategy**:
- Pattern-based grouping (feat, fix, docs, chore, etc.)
- Conventional commits format support
- Fallback to generic "changes" grouping

```rust
pub type GroupedCommits = BTreeMap<String, Vec<String>>;

pub fn group_commits(
    commits: Vec<Commit>,
    conventional: bool,
) -> Result<GroupedCommits>
```

### 5. **Infrastructure Layer** (`src/infrastructure/`)

External integrations and adapters.

#### **Git Adapter** (`git.rs`)
- Implements `GitRepository` trait
- Uses `git2` crate for Git2 functionality
- Retrieves commits from repository

#### **GitHub API Client** (`github.rs`)
- Fetches pull request information
- Generates compare links
- Optional integration

#### **Template Engine** (`template.rs`, `templates.rs`)
- Handlebars template support
- Custom and default templates
- Template data serialization

#### **Plugin System** (`plugins.rs`)
- Dynamic library loading with `libloading`
- Plugin trait definition
- Extensible architecture

#### **Workspace Detection** (`workspace.rs`)
- Detects Cargo workspace
- Multi-crate project support

### 6. **Configuration Layer** (`src/config/`)

Configuration file management.

**Supported Formats**:
- TOML (preferred)
- YAML
- JSON

**Configuration Schema**:
```rust
pub struct Config {
    pub project: Option<Project>,           // Project metadata
    pub notifications: Option<Notifications>, // Slack, Discord
    pub telemetry: Option<Telemetry>,       // Analytics
}
```

### 7. **Bootstrap Module** (`src/bootstrap.rs`)

Initialization and tracing setup.

**Features**:
- Tracing subscriber initialization
- JSON or plain text logging
- Environment filter support
- Structured logging

### 8. **Services Layer** (`src/services/`)

Business logic orchestration.

- **Generator**: Changelog generation service
- **Notifier**: Send notifications (Slack, Discord)
- **ReleaseService**: Version bumping and release
- **Telemetry**: Analytics and metrics

### 9. **Traits** (`src/traits/`)

Abstraction points for extensibility.

```rust
#[async_trait]
pub trait GitRepository: Send + Sync {
    async fn get_commits(&self) -> Result<Vec<Commit>>;
}

#[async_trait]
pub trait TemplateRenderer: Send + Sync {
    async fn render(&self, commits: Vec<Commit>) -> Result<String>;
}
```

---

## Error Handling

### Architecture

```
┌─────────────────────────────────────┐
│      CLI (main.rs)                  │
├─────────────────────────────────────┤
│ matches ChangelogError              │
│ ├─ Exit code                        │
│ ├─ Error message                    │
│ └─ std::process::exit()             │
├─────────────────────────────────────┤
│   Application Layer                 │
│   (Returns Result<T, ChangelogError>)
├─────────────────────────────────────┤
│   Domain & Services                 │
│   (Pure functions & logic)          │
├─────────────────────────────────────┤
│   Infrastructure                    │
│   (Error conversion & adaptation)   │
└─────────────────────────────────────┘
```

### Error Propagation

```rust
// Main handles all errors
#[tokio::main]
async fn main() {
    if let Err(e) = changelog_gen::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

// Library returns Result<T>
pub async fn run() -> crate::Result<()> {
    crate::application::commands::execute().await
}
```

---

## Data Flow

### Changelog Generation Flow

```
1. CLI Input Parsing
   └─ clap parses arguments → Commands enum

2. Configuration Loading
   └─ config/loader.rs loads TOML/YAML/JSON

3. Repository Access
   └─ infrastructure/git.rs → git2::Repository

4. Commit Retrieval
   └─ GitRepository trait implementation
   └─ Returns Vec<Commit>

5. Parsing & Grouping
   └─ parser.rs groups commits
   └─ Conventional commits format support
   └─ Returns: BTreeMap<String, Vec<String>>

6. Domain Logic
   └─ domain/changelog.rs generates content
   └─ Applies version and grouping

7. Rendering
   └─ Template engine (Handlebars)
   └─ Renders grouped commits to markdown

8. Output
   └─ File I/O or stdout
   └─ Success or error propagation

9. Optional: GitHub Integration
   └─ Fetch PR info via GitHub API
   └─ Generate compare links
```

### Error Flow

```
Any Layer → Result<T, ChangelogError>
   ↓
Application Layer catches error
   ↓
Error matched or propagated
   ↓
CLI handles via exit_code()
   ↓
Process exits with appropriate code
```

---

## Module Responsibilities

| Module | Responsibility | Testable | Async |
|--------|-----------------|----------|-------|
| `error` | Error definitions & exit codes | ✅ Yes | ❌ No |
| `cli` | Argument parsing & dispatch | ✅ Yes | ❌ No |
| `domain` | Pure business logic | ✅ Yes | ❌ No |
| `parser` | Commit parsing & grouping | ✅ Yes | ❌ No |
| `config` | Configuration loading | ✅ Yes | ❌ No |
| `infrastructure/git` | Git2 adapter | ✅ Yes | ✅ Yes |
| `infrastructure/github` | GitHub API client | ✅ Yes | ✅ Yes |
| `infrastructure/template` | Template rendering | ✅ Yes | ✅ Yes |
| `services` | Service orchestration | ✅ Yes | ✅ Yes |
| `traits` | Abstraction interfaces | ⚠️ Via impl | ✅ Async |
| `bootstrap` | Initialization | ⚠️ Side effects | ❌ No |

---

## Design Patterns

### 1. **Adapter Pattern**
- `GitRepository` trait with `Git2Repository` implementation
- `TemplateRenderer` trait with `HandlebarsRenderer` implementation
- Allows swapping implementations without changing business logic

### 2. **Repository Pattern**
- Abstracts Git data source
- Enables testing with mock implementations
- Single source of truth for Git operations

### 3. **Builder Pattern**
- CLI arguments construct command objects
- Configuration objects built from files
- Fluent API for configuration

### 4. **Strategy Pattern**
- Different parsing strategies (conventional vs. simple)
- Template selection strategies
- Configurable notification strategies

### 5. **Facade Pattern**
- `commands.rs` provides unified command interface
- Services orchestrate complex operations
- Simplifies client code

---

## CI/CD Architecture

### Continuous Integration Pipeline (`.github/workflows/ci.yml`)

```
Event: Push/PR to main/develop
   ↓
┌─────────────────────────────────────┐
│   Test Suite (Matrix)               │
├─────────────────────────────────────┤
│ ├─ OS: [Ubuntu, macOS, Windows]    │
│ └─ Rust: [stable, beta]            │
│   ├─ cargo test --all              │
│   ├─ cargo test --doc              │
│   └─ cargo build --release         │
└─────────────────────────────────────┘
   ↓
┌─────────────────────────────────────┐
│   Code Quality Checks               │
├─────────────────────────────────────┤
│ ├─ cargo fmt --check               │
│ ├─ cargo clippy                    │
│ └─ cargo audit (security)          │
└─────────────────────────────────────┘
   ↓
┌─────────────────────────────────────┐
│   Coverage & Artifacts              │
├─────────────────────────────────────┤
│ ├─ cargo tarpaulin (coverage)      │
│ └─ codecov.io upload               │
└─────────────────────────────────────┘
   ↓
✅ All checks pass or ❌ Fail fast
```

### Release Pipeline (`.github/workflows/release.yml`)

```
Event: Tag push (v*) or manual trigger
   ↓
┌─────────────────────────────────────┐
│   Validation Stage                  │
├─────────────────────────────────────┤
│ ├─ All tests must pass             │
│ ├─ Format check                    │
│ ├─ Clippy validation               │
│ └─ Security audit                  │
└─────────────────────────────────────┘
   ↓ (only if validation passes)
┌─────────────────────────────────────┐
│   Build Binaries (4 platforms)     │
├─────────────────────────────────────┤
│ ├─ Linux x86_64                    │
│ ├─ macOS x86_64                    │
│ ├─ macOS ARM64                     │
│ └─ Windows x86_64                  │
└─────────────────────────────────────┘
   ↓
┌─────────────────────────────────────┐
│   Create Release                    │
├─────────────────────────────────────┤
│ ├─ GitHub release page             │
│ ├─ Upload platform binaries        │
│ └─ Tag with version                │
└─────────────────────────────────────┘
   ↓
┌─────────────────────────────────────┐
│   Publish (on tag)                  │
├─────────────────────────────────────┤
│ └─ cargo publish → crates.io       │
└─────────────────────────────────────┘
   ↓
✅ Published and ready for distribution
```

---

## Build & Release

### Build Profiles

**Debug** (`cargo build`)
```toml
[profile.dev]
opt-level = 0
debug = true
```

**Release** (`cargo build --release`)
```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Slower build, better optimization
strip = true           # Remove debug symbols
```

**Test** (`cargo test`)
```toml
[profile.test]
opt-level = 1          # Some optimization
```

### Release Process

1. **Version Bump**
   - Update `Cargo.toml` version
   - Update `CHANGELOG.md`

2. **Commit & Tag**
   ```bash
   git tag v0.1.0
   git push origin main --tags
   ```

3. **CI/CD Automation**
   - Tests run on all platforms
   - Binaries built for 4 platforms
   - GitHub release created
   - Published to crates.io

4. **Publication**
   - Available via `cargo install changelog-gen`
   - Binaries on GitHub releases
   - Documentation on docs.rs

---

## Security Architecture

### Dependency Security

```
┌─────────────────────────────────────┐
│   Cargo Manifest                    │
├─────────────────────────────────────┤
│ ├─ Direct dependencies pinned      │
│ ├─ Transitive scanning             │
│ └─ Well-maintained crates          │
└─────────────────────────────────────┘
   ↓
┌─────────────────────────────────────┐
│   CI/CD Security Check              │
├─────────────────────────────────────┤
│ ├─ `cargo audit` on every push     │
│ ├─ CVE database updated            │
│ └─ Fails if vulnerabilities found  │
└─────────────────────────────────────┘
   ↓
✅ Only non-vulnerable dependencies
```

### Error Security

- No sensitive data in error messages
- Structured error types prevent info leaks
- Proper exit codes for monitoring
- Logging with configured filters

### Code Security

- Type safety prevents common bugs
- No unsafe blocks in user code
- Format validation with regex
- Input validation at boundaries

---

## Extension Points

### 1. **Custom GitRepository Implementation**
```rust
#[async_trait]
impl GitRepository for CustomVCS {
    async fn get_commits(&self) -> Result<Vec<Commit>> {
        // Custom VCS implementation
    }
}
```

Supports:
- GitHub (via API)
- GitLab
- Bitbucket
- Custom Git servers

### 2. **Custom Template Rendering**
```rust
#[async_trait]
impl TemplateRenderer for CustomRenderer {
    async fn render(&self, commits: Vec<Commit>) -> Result<String> {
        // Custom rendering logic
    }
}
```

Supports:
- Handlebars (default)
- Tera templates
- Custom markdown generation
- HTML output
- JSON output

### 3. **Plugin System**
- Dynamic library loading
- Plugin trait interface
- Runtime plugin discovery
- Custom processing pipelines

### 4. **Configuration Extensions**
```toml
[project]
name = "my-project"
custom_field = "value"

[notifications]
# Add custom notification channels
```

### 5. **Custom Commands**
- CLI subcommand registration
- Additional services
- Custom validation rules

---

## Dependencies

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime |
| `clap` | 4.x | CLI arg parsing |
| `git2` | 0.18 | Git operations |
| `serde` | 1.x | Serialization |
| `thiserror` | 1.x | Error handling |
| `chrono` | 0.4 | Date/time |
| `regex` | 1.x | Pattern matching |
| `handlebars` | 5.x | Templates |
| `reqwest` | 0.11 | HTTP client |
| `tracing` | 0.1 | Structured logging |

### Feature Flags

```toml
# No optional features in core
# Extensibility through traits and plugins
```

---

## Async Architecture

```
Main Entry
   ↓
#[tokio::main] → async runtime
   ↓
commands::execute() → async
   ↓
Multiple async operations
   ├─ Git repository access
   ├─ GitHub API calls (optional)
   ├─ Template rendering
   └─ File I/O
   ↓
Result aggregation
   ↓
Error handling & exit
```

**Concurrency Model**:
- Tokio runtime (multi-threaded by default)
- Async/await for I/O-bound operations
- No data races (Rust's type system)

---

## Testing Architecture

### Test Levels

```
┌────────────────────────────────────┐
│   Unit Tests (src/error.rs)        │
│   ├─ Error exit codes              │
│   ├─ Error recovery classification │
│   └─ Error display formatting      │
└────────────────────────────────────┘
   ↓
┌────────────────────────────────────┐
│   Integration Tests (tests/)       │
│   ├─ Changelog generation          │
│   ├─ Config loading                │
│   ├─ Parser functionality          │
│   └─ Version utilities             │
└────────────────────────────────────┘
   ↓
┌────────────────────────────────────┐
│   CI/CD Tests (multi-platform)     │
│   ├─ Ubuntu, macOS, Windows        │
│   ├─ Stable & beta Rust            │
│   └─ Code coverage reporting       │
└────────────────────────────────────┘
```

### Coverage Goals

- **Error module**: 100%
- **Parser module**: 90%+
- **Domain logic**: 85%+
- **Infrastructure**: 70%+ (with mocks)
- **Overall**: >80%

---

## Development Workflow

### Local Development

```bash
# Setup
git clone https://github.com/SaumilP/changelog-gen.git
cd changelog-gen
cargo build

# Development cycle
cargo test --all              # Run tests
cargo fmt                     # Format code
cargo clippy                  # Lint
RUST_LOG=debug cargo run      # Run with logging

# Before commit
cargo test --all
cargo fmt -- --check
cargo clippy -- -D warnings
```

### Branch Strategy

- **`main`**: Stable releases only
- **`develop`**: Development for next release
- **Feature branches**: From `develop`, PR to `develop`
- **Release branches**: Prepare release, PR to `main`

### Commit Convention

```
<type>(<scope>): <subject>

Types: feat, fix, docs, style, refactor, test, chore
Example: feat(parser): add conventional commit support
```

---

## Performance Considerations

### Optimization Strategies

1. **Compilation**
   - Release profile with LTO
   - Single codegen unit
   - Link-time optimization

2. **Runtime**
   - Async I/O for Git operations
   - Streaming for large repositories
   - Lazy template compilation

3. **Memory**
   - Vec/BTreeMap for collections
   - String references where possible
   - Minimal allocations in hot paths

### Benchmarking

```bash
# Future: cargo bench
# Measure:
# - Git parsing speed
# - Template rendering
# - Config loading
# - End-to-end generation
```

---

## Maintenance & Updates

### Dependency Management

- Regular audits (`cargo audit`)
- Timely updates to stable versions
- Test compatibility with new versions
- Pin to stable, avoid alpha/beta

### Version Support

- Current: 0.1.0 (active development)
- Roadmap: 0.2.0, 0.3.0, 1.0.0
- Long-term support after 1.0.0

### Security Patches

- Released immediately
- Patch version bump
- CI/CD automated distribution
- Notification to users

---

## Summary

changelog-gen's architecture prioritizes:

✅ **Modularity** - Clear separation of concerns  
✅ **Type Safety** - Rust's type system + custom errors  
✅ **Testability** - Pure functions + trait abstractions  
✅ **Extensibility** - Plugins + trait implementations  
✅ **Maintainability** - Organized code structure  
✅ **Security** - Integrated scanning + secure patterns  
✅ **Performance** - Optimized builds + async I/O  
✅ **Reliability** - Comprehensive error handling  

This architecture supports production use and enables future growth and enhancements.
