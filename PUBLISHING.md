# Publishing Guide for changelog-gen

This document provides comprehensive instructions for publishing and releasing changelog-gen as a production-ready Rust crate.

## 📋 Pre-Release Checklist

Before publishing a new version, ensure:

- [ ] All tests pass: `cargo test --all`
- [ ] Code formatting is correct: `cargo fmt -- --check`
- [ ] No clippy warnings: `cargo clippy --all-targets -- -D warnings`
- [ ] Security audit passes: `cargo audit`
- [ ] Documentation is complete and accurate
- [ ] CHANGELOG.md is updated with all changes
- [ ] Version number is updated in Cargo.toml
- [ ] Git is clean with no uncommitted changes

## 🔑 Setup Credentials

### Crates.io Token

1. Create an account at [crates.io](https://crates.io)
2. Generate an API token from [Account Settings](https://crates.io/me)
3. Create GitHub Secret `CARGO_REGISTRY_TOKEN`:
   ```bash
   gh secret set CARGO_REGISTRY_TOKEN --body "YOUR_TOKEN"
   ```

### Manual Publishing

```bash
# Login to crates.io (one-time setup)
cargo login

# Your token will be stored in ~/.cargo/credentials.toml
```

## 📦 Publishing Process

### Automated Release (Recommended)

The release workflow is fully automated via GitHub Actions:

1. **Update Version**

   In `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Bump version
   ```

2. **Update Changelog**

   Append to `CHANGELOG.md`:
   ```markdown
   ## [0.2.0] - YYYY-MM-DD

   ### Added
   - New feature description

   ### Fixed
   - Bug fix description
   ```

3. **Commit and Tag**

   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: release v0.2.0"
   git tag v0.2.0
   git push origin main --tags
   ```

4. **Monitor Release**

   - GitHub Actions will automatically:
     - Run full test suite on multiple platforms
     - Build binaries for Linux, macOS, and Windows
     - Create GitHub release with asset downloads
     - Publish to crates.io

### Manual Release

If automated release fails:

1. Publish to crates.io:
   ```bash
   cargo publish
   ```

2. Create GitHub release manually:
   - Go to [Releases](https://github.com/SaumilP/changelog-gen/releases)
   - Click "Create a new release"
   - Enter tag `v0.2.0`
   - Upload built binaries

## 🎯 Version Strategies

### Semantic Versioning Examples

**0.1.0 → 0.2.0** (MINOR - feature addition)
```bash
git tag v0.2.0
```

**0.2.0 → 0.2.1** (PATCH - bug fix)
```bash
git tag v0.2.1
```

**0.2.1 → 1.0.0** (MAJOR - stable release)
```bash
git tag v1.0.0
```

**1.0.0 → 1.1.0-alpha.1** (Pre-release)
```bash
git tag v1.1.0-alpha.1
```

## 📊 Publication Status

### Verification Steps

```bash
# Check it's on crates.io
curl https://crates.io/api/v1/crates/changelog-gen | jq '.crate.max_version'

# Check documentation is built
curl https://docs.rs/changelog-gen/latest/changelog_gen/ --head

# Verify release on GitHub
gh release view v0.2.0
```

### First-Time Publishing

The first time publishing to crates.io requires:

1. Package name must be unique
2. Package metadata must be complete:
   - Clear description
   - License specified
   - Repository URL
   - Documentation URL
   - Keywords and categories

## 🔐 Security Considerations

### Before Publishing

- [ ] Run `cargo audit` to check dependencies
- [ ] Review all exposed public APIs
- [ ] Check for hardcoded credentials or secrets
- [ ] Ensure `.gitignore` is properly configured
- [ ] Remove any test credentials or mock data

### Maintenance

- [ ] Monitor security advisories
- [ ] Subscribe to crate update notifications
- [ ] Regularly audit dependencies
- [ ] Release security patches promptly

## 📝 Cargo.toml Publishing Metadata

```toml
[package]
name = "changelog-gen"
version = "0.1.0"
edition = "2021"
authors = ["Author Name <email@example.com>"]
license = "MIT"
description = "Production-grade Rust CLI to generate changelogs from Git history"
repository = "https://github.com/SaumilP/changelog-gen"
homepage = "https://github.com/SaumilP/changelog-gen"
documentation = "https://docs.rs/changelog-gen"
readme = "README.md"
keywords = ["changelog", "git", "devops", "release", "cli"]
categories = ["command-line-utilities", "development-tools"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## 🔄 Post-Release Tasks

After successful release:

1. **Announce Release**
   - Post on GitHub Discussions
   - Update project social media
   - Notify subscribers

2. **Update Documentation**
   - Verify docs.rs page is updated
   - Update installation instructions
   - Create blog post if major release

3. **Monitor Feedback**
   - Check GitHub issues
   - Monitor crates.io comments
   - Fix any reported issues promptly

## 🚧 Pre-Release Versions

For alpha, beta, or release candidates:

```bash
# Alpha release
git tag v1.0.0-alpha.1

# Beta release
git tag v1.0.0-beta.1

# Release candidate
git tag v1.0.0-rc.1
```

These will be published to crates.io but marked as pre-releases.

## 📞 Troubleshooting

### "Package already exists"
The version already exists. Bump version and try again.

### "Size exceeds limits"
Exclude unnecessary files in `Cargo.toml`:
```toml
exclude = [".github", "tests", "examples"]
```

### "Documentation build failed"
Check docs.rs build logs. Often due to missing feature flags or API documentation.

### "Binary upload failed"
Ensure artifact names match and are under size limit.

## 📚 Additional Resources

- [Publishing on crates.io](https://doc.rust-lang.org/cargo/publish.html)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)

## 🎉 Release Templates

### Release Announcement

```
🎉 Version 0.2.0 Released!

✨ New Features:
- Feature 1
- Feature 2

🐛 Bug Fixes:
- Fixed issue #123
- Fixed issue #456

📦 Downloads:
- GitHub Releases: [Link]
- crates.io: `cargo install changelog-gen@0.2.0`

📖 Documentation: https://docs.rs/changelog-gen/0.2.0

Thank you for using changelog-gen!
```

---

**Last Updated**: 2026-02-17  
**Maintainer**: Saumil Patel
