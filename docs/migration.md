# Migration notes

## From pre-core-workflow builds

This release introduces a new CLI contract and core changelog document engine.

### Command changes

- `init` has been replaced by `new` for changelog scaffolding.
- `generate` now supports `--since`, `--until`, `--specific`, `--milestone`, `--map`, and `--output`.
- `release` now supports explicit `--version` or `--bump` and configurable `--header`.
- new `show` and `remove` commands are available.

### Behavior changes

- Release ordering is SemVer-based.
- Notes are deduplicated in both `generate` and `release`.
- Ignore markers in commit messages are enforced.
- Validation errors now include location and fix guidance.
