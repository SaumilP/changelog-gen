# Changelog format

`changeloggen-cli` reads and writes Markdown with this structure:

```markdown
# Changelog

## [1.2.0] - 2026-02-23

### Added
- user-visible change

### Fixed
- bug fix detail
```

## Rules

- The first heading must be `# Changelog`.
- Releases must use SemVer (`x.y.z`).
- Releases are sorted descending by SemVer.
- Notes must appear under a `### <Section>` heading.
- Notes must use `- <text>` bullets.

## Header styles

Release headers can be configured in `release --header`:

- `default` / `brackets`: `## [x.y.z] - YYYY-MM-DD`
- `plain`: `## x.y.z - YYYY-MM-DD`
- `version-only`: `## [x.y.z]`
- custom template with `{version}` and `{date}`

## Validation errors

Invalid files report:

- exact line location
- expected token/structure
- found input
- fix guidance

Example error shape:

```
Invalid changelog at line 12: expected a section heading before notes, found - orphan note. Fix: insert a heading like '### Added' above this note
```
