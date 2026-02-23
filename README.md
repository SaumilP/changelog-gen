[![Crates.io](https://img.shields.io/crates/v/changeloggen-cli.svg)](https://crates.io/crates/changeloggen-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# changeloggen-cli

`changeloggen-cli` is a Rust CLI for scaffolding, validating, generating, and managing changelog releases from git history.

## Install

```bash
cargo install changeloggen-cli
```

GitHub Releases archives are also published in `changeloggen-cli-{target}.tar.gz` / `.zip` format for `cargo-binstall` compatibility.

## Core commands

### `new`

Create a starter changelog file.

```bash
changeloggen-cli new --file CHANGELOG.md --format markdown
```

### `validate`

Validate changelog syntax/semantics (non-zero exit on invalid).

```bash
changeloggen-cli validate --file CHANGELOG.md --strict
```

### `generate`

Generate release notes from git history.

```bash
changeloggen-cli generate \
  --file CHANGELOG.md \
  --since v1.2.0 \
  --until HEAD \
  --map changelog-map.toml \
  --output notes.md
```

Range modes:

- `--since <tag|sha>`
- `--until <tag|sha>`
- `--specific <tag|sha>`
- `--milestone <name|id>` (requires `--github`; currently returns clear unsupported error)

### `release`

Create/update a release entry in `CHANGELOG.md` using generated notes.

```bash
changeloggen-cli release --version 1.4.0 --file CHANGELOG.md --header default
changeloggen-cli release --bump patch --file CHANGELOG.md --header plain
changeloggen-cli release --version 1.4.0 --override
```

Header presets:

- `default` / `brackets`: `## [x.y.z] - YYYY-MM-DD`
- `plain`: `## x.y.z - YYYY-MM-DD`
- `version-only`: `## [x.y.z]`
- custom template string supporting `{version}` and `{date}`

### `show`

Show releases from a changelog file.

```bash
changeloggen-cli show --file CHANGELOG.md --version 1.4.0
changeloggen-cli show --file CHANGELOG.md --range 1.2.0..1.4.0
changeloggen-cli show --file CHANGELOG.md --range 1.2.0..1.4.0 --converge
```

`--converge` merges selected releases into one deduplicated view.

### `remove`

Remove a release from changelog by version.

```bash
changeloggen-cli remove --version 1.2.3 --file CHANGELOG.md --yes
```

## Commit handling rules

- Conventional commits are mapped to changelog sections.
- Ignore markers skip commits entirely:
  - `(skip changelog)`
  - `(ignore changelog)`
  - `!changelog`
  - `!log`
- Notes are deduplicated deterministically in both `generate` and `release`.

## Type mapping override

Pass a map file with `--map` (`.json` or `.toml`) to override commit type -> section.

`changelog-map.toml` example:

```toml
[types]
feat = "Features"
fix = "Bug Fixes"
perf = "Performance"
```

## CI usage

```bash
changeloggen-cli validate --strict
changeloggen-cli generate --since v1.2.0 --until HEAD --output release-notes.md
```

More details:

- `docs/changelog-format.md`
- `docs/ci-usage.md`
- `docs/migration.md`
