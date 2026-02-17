# Versioning Strategy

This project follows **Semantic Versioning** (SemVer) as defined in [semver.org](https://semver.org/).

## Version Format

All releases follow the format: `MAJOR.MINOR.PATCH[-PRERELEASE][+METADATA]`

Examples:
- `0.1.0` - Initial release
- `1.0.0` - First stable release
- `1.1.0` - Feature addition
- `1.1.1` - Bug fix
- `2.0.0` - Breaking changes
- `1.0.0-alpha` - Pre-release version
- `1.0.0-rc.1` - Release candidate

## Versioning Rules

### MAJOR Version (X.0.0)
Increment when making **incompatible API changes**:
- Removing or renaming public functions
- Changing function signatures
- Modifying error types significantly
- Breaking changes to CLI interface

### MINOR Version (0.X.0)
Increment when adding **backwards-compatible functionality**:
- New public functions or methods
- New CLI options that don't break existing commands
- Performance improvements
- New features

### PATCH Version (0.0.X)
Increment for **backwards-compatible bug fixes**:
- Bug fixes
- Security patches
- Documentation corrections
- Internal refactoring

## Release Process

### Automated Release (Recommended)

1. **Prepare Release:**
   ```bash
   # Ensure all tests pass
   cargo test --all
   
   # Update version in Cargo.toml
   # Update CHANGELOG.md
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: release v1.2.3"
   git tag v1.2.3
   ```

2. **Push to Trigger Release:**
   ```bash
   git push origin main --tags
   ```

3. The GitHub Actions workflow will:
   - Validate all tests pass
   - Build binaries for multiple platforms
   - Create GitHub release with assets
   - Publish to crates.io

### Manual Workflow Dispatch

Alternatively, use GitHub's manual workflow dispatch:

1. Go to Actions → Release and Publish
2. Click "Run workflow"
3. Enter version number (e.g., `1.2.3`)
4. Workflow executes automatically

## Pre-release Versions

For alpha, beta, or release candidate versions:

```bash
git tag v1.0.0-alpha.1
git tag v1.0.0-beta.1
git tag v1.0.0-rc.1
```

Pre-releases are created with `draft: false` but marked clearly in GitHub releases.

## Changelog Management

Maintain `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
## [1.2.0] - 2026-02-17

### Added
- New feature description

### Changed
- Modified feature description

### Fixed
- Bug fix description

### Removed
- Removed feature description

### Deprecated
- Deprecated feature description

### Security
- Security fix description
```

## Branch Strategy

- `main` - Stable releases only
- `develop` - Development branch for next release
- Feature branches created from `develop`
- PRs merged to `develop` then to `main` for releases

## Version History

| Version | Release Date | Status |
|---------|-------------|--------|
| 0.1.0 | 2026-02-17 | Initial Release |

## Future Roadmap

- v0.2.0: Enhanced GitHub integration
- v0.3.0: Plugin system improvements
- v1.0.0: Stable API release
