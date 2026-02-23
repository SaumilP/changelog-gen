# CI usage

Use `changeloggen-cli` in CI to enforce format quality and generate release notes.

## Validate changelog

```bash
changeloggen-cli validate --file CHANGELOG.md --strict
```

Exit code is non-zero when the changelog is invalid.

## Generate notes for a range

```bash
changeloggen-cli generate --since v1.5.0 --until HEAD --output release-notes.md
```

## Suggested GitHub Actions step

```yaml
- name: Validate changelog
  run: changeloggen-cli validate --strict

- name: Generate release notes
  run: changeloggen-cli generate --since ${{ github.event.release.tag_name }} --until HEAD --output release-notes.md
```

## Release packaging and cargo-binstall

The release workflow publishes one archive per target:

- `changeloggen-cli-x86_64-unknown-linux-gnu.tar.gz`
- `changeloggen-cli-x86_64-apple-darwin.tar.gz`
- `changeloggen-cli-aarch64-apple-darwin.tar.gz`
- `changeloggen-cli-x86_64-pc-windows-msvc.zip`

Each archive contains only the intended binary (`changeloggen-cli` / `changeloggen-cli.exe`) and is checked by a binstall smoke-check job.
