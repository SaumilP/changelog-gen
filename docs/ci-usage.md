# CI usage

Use `changelog-gen` in CI to enforce format quality and generate release notes.

## Validate changelog

```bash
changelog-gen validate --file CHANGELOG.md --strict
```

Exit code is non-zero when the changelog is invalid.

## Generate notes for a range

```bash
changelog-gen generate --since v1.5.0 --until HEAD --output release-notes.md
```

## Suggested GitHub Actions step

```yaml
- name: Validate changelog
  run: changelog-gen validate --strict

- name: Generate release notes
  run: changelog-gen generate --since ${{ github.event.release.tag_name }} --until HEAD --output release-notes.md
```

## Release packaging and cargo-binstall

The release workflow publishes one archive per target:

- `changelog-gen-x86_64-unknown-linux-gnu.tar.gz`
- `changelog-gen-x86_64-apple-darwin.tar.gz`
- `changelog-gen-aarch64-apple-darwin.tar.gz`
- `changelog-gen-x86_64-pc-windows-msvc.zip`

Each archive contains only the intended binary (`changelog-gen` / `changelog-gen.exe`) and is checked by a binstall smoke-check job.
