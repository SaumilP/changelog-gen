[![Crates.io](https://img.shields.io/crates/v/changelog-gen.svg)](https://crates.io/crates/changelog-gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# changelog-gen

`changelog-gen` is a Rust CLI for scaffolding, validating, generating, and managing changelog releases from git history.

## Install

```bash
cargo install changelog-gen
```

GitHub Releases archives are also published in `changelog-gen-{target}.tar.gz` / `.zip` format for `cargo-binstall` compatibility.

## Core commands

### `new`

Create a starter changelog file.

```bash
changelog-gen new --file CHANGELOG.md --format markdown
```

### `validate`

Validate changelog syntax/semantics (non-zero exit on invalid).

```bash
changelog-gen validate --file CHANGELOG.md --strict
```

### `generate`

Generate release notes from git history.

```bash
changelog-gen generate \
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
changelog-gen release --version 1.4.0 --file CHANGELOG.md --header default
changelog-gen release --bump patch --file CHANGELOG.md --header plain
changelog-gen release --version 1.4.0 --override
```

Header presets:

- `default` / `brackets`: `## [x.y.z] - YYYY-MM-DD`
- `plain`: `## x.y.z - YYYY-MM-DD`
- `version-only`: `## [x.y.z]`
- custom template string supporting `{version}` and `{date}`

### `show`

Show releases from a changelog file.

```bash
changelog-gen show --file CHANGELOG.md --version 1.4.0
changelog-gen show --file CHANGELOG.md --range 1.2.0..1.4.0
changelog-gen show --file CHANGELOG.md --range 1.2.0..1.4.0 --converge
```

`--converge` merges selected releases into one deduplicated view.

### `remove`

Remove a release from changelog by version.

```bash
changelog-gen remove --version 1.2.3 --file CHANGELOG.md --yes
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
changelog-gen validate --strict
changelog-gen generate --since v1.2.0 --until HEAD --output release-notes.md
```

More details:

- `docs/changelog-format.md`
- `docs/ci-usage.md`
- `docs/migration.md`
