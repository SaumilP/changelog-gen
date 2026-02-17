[![Crates.io](https://img.shields.io/crates/v/changelog-gen.svg)](https://crates.io/crates/changelog-gen)
[![Docs.rs](https://docs.rs/changelog-gen/badge.svg)](https://docs.rs/changelog-gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/SaumilP/changelog-gen/workflows/Continuous%20Integration/badge.svg)](https://github.com/SaumilP/changelog-gen/actions)
[![codecov](https://codecov.io/gh/SaumilP/changelog-gen/branch/main/graph/badge.svg)](https://codecov.io/gh/SaumilP/changelog-gen)

# changelog-gen

`changelog-gen` is a Rust CLI that builds changelogs from your Git history.

It works well with conventional commits, can add GitHub compare/PR context, and supports custom templates when you want to control the final output instead of hand-editing `CHANGELOG.md` at midnight.

## Why use it?

- Groups commits in a way humans can actually read.
- Understands conventional commit prefixes (`feat`, `fix`, etc.).
- Can generate release notes with GitHub context.
- Supports semantic version bumping.
- Lets you use Handlebars templates for custom output.
- Reads config from TOML/YAML/JSON.
- Has a plugin-friendly architecture if you want to extend behavior.

## Install

From crates.io:

```bash
cargo install changelog-gen
```

From source:

```bash
git clone https://github.com/SaumilP/changelog-gen.git
cd changelog-gen
cargo install --path .
```

Or grab a binary from [GitHub Releases](https://github.com/SaumilP/changelog-gen/releases).

## Quick start

```bash
# Create default config
changelog-gen init

# Generate changelog for a release
changelog-gen generate --release 1.0.0

# Same, but include GitHub data
changelog-gen generate --release 1.0.0 --github

# Bump version + generate release changelog
changelog-gen release --bump minor
```

New to Rust tooling? You can still get productive quickly:

- `cargo install changelog-gen` installs the CLI globally.
- `cargo run -- <args>` runs it from source without installing.
- `cargo test` runs tests.
- `cargo fmt` and `cargo clippy` are the standard formatting/lint checks you’ll see in CI.

## Commands

### `init`

Creates a default `changelog.toml` in the current directory.

```bash
changelog-gen init
```

### `generate`

Generates changelog content from commit history.

```bash
changelog-gen generate [OPTIONS]
```

Options:

- `--release <VERSION>`: release version like `1.2.3`
- `--github`: include GitHub compare / PR details
- `--conventional`: parse commit messages as conventional commits
- `--template <PATH>`: custom Handlebars template path
- `--output <PATH>`: output file path (default: `CHANGELOG.md`)

Examples:

```bash
changelog-gen generate --release 2.0.0
changelog-gen generate --release 2.0.0 --github
changelog-gen generate --release 2.0.0 --conventional
changelog-gen generate --release 2.0.0 --template ./my-template.hbs
```

### `release`

Bumps version and generates a changelog in one go.

```bash
changelog-gen release [OPTIONS]
```

Options:

- `--bump <TYPE>`: `major`, `minor`, or `patch`

Examples:

```bash
changelog-gen release --bump patch
changelog-gen release --bump minor
changelog-gen release --bump major
```

## Configuration

Put one of these files in your project root:

- `changelog.toml`
- `changelog.yaml`
- `changelog.yml`

Example (`changelog.toml`):

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

## Development

```bash
git clone https://github.com/SaumilP/changelog-gen.git
cd changelog-gen

cargo build
cargo test --all
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## If You’re New to Rust

This repo is friendly to start with because the CLI surface is small and you can test changes fast.

Suggested first path:

1. Run `cargo test --all` to confirm your environment works.
2. Open `src/main.rs` and `src/cli/` to see how commands are wired.
3. Run `cargo run -- generate --release 0.1.0` in a sample Git repo.
4. Make a small change (for example, output text in `generate`) and re-run tests.

If a command fails with a long compiler error, scroll to the first real `error:` line. Rust tends to report the useful part early, then keeps talking.

## If You Maintain This Project

Day-to-day expectations:

- Keep CI green: `fmt`, `clippy`, tests, audit, and release build.
- Treat `-D warnings` as a contract, not a suggestion.
- Prefer small PRs with tests over large refactors without coverage.
- Keep command examples in this README runnable.

Release and publish details live in [PUBLISHING.md](PUBLISHING.md), and the release workflow is in `.github/workflows/release.yml`.

## Security

Please read [SECURITY.md](SECURITY.md) for reporting details.

## Docs and project files

- API docs: [docs.rs/changelog-gen](https://docs.rs/changelog-gen)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)
- Versioning notes: [VERSIONING.md](VERSIONING.md)
- Publishing flow: [PUBLISHING.md](PUBLISHING.md)
- Project changelog: [CHANGELOG.md](CHANGELOG.md)

## Contributing

PRs are welcome. Small fixes, docs cleanup, and tests are all useful contributions, not just big feature work.

If you’re changing behavior, include tests. If you’re touching docs, keep examples runnable.

## License

MIT. See [LICENSE](LICENSE).

## Support

- Issues: [GitHub Issues](https://github.com/SaumilP/changelog-gen/issues)
- Discussions: [GitHub Discussions](https://github.com/SaumilP/changelog-gen/discussions)

## Roadmap (rough, not a blood oath)

- v0.2.0: Better GitHub workflow templates
- v0.3.0: Docker support and container publishing
- v1.0.0: Stable API release
- v1.1.0: Performance benchmarking

Built by [Saumil Patel](https://github.com/SaumilP).
