use chrono::Utc;
use semver::Version;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangelogDocument {
    pub title: String,
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub version: Version,
    pub date: Option<String>,
    pub header: HeaderFormat,
    pub sections: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderFormat {
    Default,
    Plain,
    VersionOnly,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIssue {
    pub line: usize,
    pub expected: String,
    pub found: String,
    pub fix: String,
}

impl ParseIssue {
    pub fn message(&self) -> String {
        format!(
            "Invalid changelog at line {}: expected {}, found {}. Fix: {}",
            self.line, self.expected, self.found, self.fix
        )
    }
}

impl ChangelogDocument {
    pub fn scaffold() -> Self {
        Self {
            title: "Changelog".to_string(),
            releases: Vec::new(),
        }
    }

    pub fn parse(input: &str) -> Result<Self, ParseIssue> {
        let lines: Vec<&str> = input.lines().collect();
        let mut idx = 0usize;

        while idx < lines.len() && lines[idx].trim().is_empty() {
            idx += 1;
        }

        if idx >= lines.len() {
            return Err(ParseIssue {
                line: 1,
                expected: "a '# Changelog' title".to_string(),
                found: "empty file".to_string(),
                fix: "create a file starting with '# Changelog'".to_string(),
            });
        }

        let title_line = lines[idx].trim();
        if !title_line.starts_with("# ") {
            return Err(ParseIssue {
                line: idx + 1,
                expected: "a level-1 heading like '# Changelog'".to_string(),
                found: title_line.to_string(),
                fix: "replace the first non-empty line with '# Changelog'".to_string(),
            });
        }

        let mut doc = ChangelogDocument {
            title: title_line.trim_start_matches("# ").to_string(),
            releases: Vec::new(),
        };

        idx += 1;

        while idx < lines.len() {
            let line = lines[idx].trim();

            if line.is_empty() {
                idx += 1;
                continue;
            }

            if !line.starts_with("## ") {
                return Err(ParseIssue {
                    line: idx + 1,
                    expected: "a release heading '## [x.y.z] - YYYY-MM-DD'".to_string(),
                    found: line.to_string(),
                    fix: "add a release heading before sections and notes".to_string(),
                });
            }

            let (version, date, header) = parse_release_heading(line, idx + 1)?;
            idx += 1;

            let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
            let mut current_section = String::new();

            while idx < lines.len() {
                let current = lines[idx].trim();
                if current.is_empty() {
                    idx += 1;
                    continue;
                }

                if current.starts_with("## ") {
                    break;
                }

                if current.starts_with("### ") {
                    current_section = current.trim_start_matches("### ").trim().to_string();
                    if current_section.is_empty() {
                        return Err(ParseIssue {
                            line: idx + 1,
                            expected: "a section title after '###'".to_string(),
                            found: current.to_string(),
                            fix: "use section headings like '### Added' or '### Fixed'".to_string(),
                        });
                    }
                    sections.entry(current_section.clone()).or_default();
                    idx += 1;
                    continue;
                }

                if !current.starts_with("- ") {
                    return Err(ParseIssue {
                        line: idx + 1,
                        expected: "a bullet note '- ...' or a section heading '### ...'".to_string(),
                        found: current.to_string(),
                        fix: "prefix notes with '- ' and group them under '### <Section>'".to_string(),
                    });
                }

                if current_section.is_empty() {
                    return Err(ParseIssue {
                        line: idx + 1,
                        expected: "a section heading before notes".to_string(),
                        found: current.to_string(),
                        fix: "insert a heading like '### Added' above this note".to_string(),
                    });
                }

                let note = current.trim_start_matches("- ").trim().to_string();
                if note.is_empty() {
                    return Err(ParseIssue {
                        line: idx + 1,
                        expected: "a non-empty note".to_string(),
                        found: current.to_string(),
                        fix: "write text after '- '".to_string(),
                    });
                }
                sections.entry(current_section.clone()).or_default().push(note);
                idx += 1;
            }

            doc.releases.push(Release {
                version,
                date,
                header,
                sections,
            });
        }

        Ok(doc)
    }

    pub fn validate(&self, strict: bool) -> Result<(), ParseIssue> {
        if self.title.trim().is_empty() {
            return Err(ParseIssue {
                line: 1,
                expected: "a non-empty title".to_string(),
                found: "empty title".to_string(),
                fix: "use '# Changelog' as the first heading".to_string(),
            });
        }

        let mut seen: BTreeSet<Version> = BTreeSet::new();
        for (index, release) in self.releases.iter().enumerate() {
            if !seen.insert(release.version.clone()) {
                return Err(ParseIssue {
                    line: index + 2,
                    expected: "unique release versions".to_string(),
                    found: format!("duplicate {}", release.version),
                    fix: "remove duplicates or merge notes into one release".to_string(),
                });
            }

            if release.sections.is_empty() && strict {
                return Err(ParseIssue {
                    line: index + 2,
                    expected: "at least one section per release in --strict mode".to_string(),
                    found: format!("release {} has no sections", release.version),
                    fix: "add a section like '### Added' with notes".to_string(),
                });
            }

            for (name, notes) in &release.sections {
                if name.trim().is_empty() {
                    return Err(ParseIssue {
                        line: index + 2,
                        expected: "non-empty section headings".to_string(),
                        found: "empty section heading".to_string(),
                        fix: "rename section to something like 'Added'".to_string(),
                    });
                }
                if strict && notes.is_empty() {
                    return Err(ParseIssue {
                        line: index + 2,
                        expected: "non-empty section notes in --strict mode".to_string(),
                        found: format!("section '{}' has no notes", name),
                        fix: "add at least one note under this section".to_string(),
                    });
                }
            }
        }

        if !is_semver_desc_sorted(&self.releases) {
            return Err(ParseIssue {
                line: 2,
                expected: "releases sorted descending by SemVer".to_string(),
                found: "out-of-order versions".to_string(),
                fix: "sort releases so highest version comes first".to_string(),
            });
        }

        Ok(())
    }

    pub fn sort_semver_desc(&mut self) {
        self.releases.sort_by(|a, b| b.version.cmp(&a.version));
    }

    pub fn remove_version(&mut self, version: &Version) -> bool {
        let previous = self.releases.len();
        self.releases.retain(|r| &r.version != version);
        self.releases.len() != previous
    }

    pub fn upsert_release(&mut self, release: Release, override_existing: bool) -> Result<(), String> {
        if let Some(idx) = self
            .releases
            .iter()
            .position(|existing| existing.version == release.version)
        {
            if !override_existing {
                return Err(format!(
                    "release {} already exists (use --override to replace it)",
                    release.version
                ));
            }
            self.releases[idx] = release;
        } else {
            self.releases.push(release);
        }

        self.sort_semver_desc();
        Ok(())
    }

    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# ");
        out.push_str(&self.title);
        out.push_str("\n\n");

        for release in &self.releases {
            out.push_str(&render_release_header(release));
            out.push_str("\n\n");

            for (section, notes) in &release.sections {
                out.push_str("### ");
                out.push_str(section);
                out.push('\n');
                for note in notes {
                    out.push_str("- ");
                    out.push_str(note);
                    out.push('\n');
                }
                out.push('\n');
            }
        }

        out.trim_end().to_string() + "\n"
    }
}

impl Release {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            date: Some(Utc::now().format("%Y-%m-%d").to_string()),
            header: HeaderFormat::Default,
            sections: BTreeMap::new(),
        }
    }

    pub fn add_note(&mut self, section: String, note: String) {
        self.sections.entry(section).or_default().push(note);
    }
}

fn parse_release_heading(
    line: &str,
    line_number: usize,
) -> Result<(Version, Option<String>, HeaderFormat), ParseIssue> {
    let rest = line.trim_start_matches("## ").trim();

    let (version_raw, date_raw, header) = if rest.starts_with('[') {
        let close = rest.find(']').ok_or_else(|| ParseIssue {
            line: line_number,
            expected: "closing ']' in release heading".to_string(),
            found: rest.to_string(),
            fix: "use heading format like '## [1.2.3] - 2026-01-01'".to_string(),
        })?;
        let version = &rest[1..close];
        let trailing = rest[close + 1..].trim();
        let date = trailing
            .strip_prefix('-')
            .map(|value| value.trim().to_string())
            .filter(|v| !v.is_empty());
        let header = if date.is_some() {
            HeaderFormat::Default
        } else {
            HeaderFormat::VersionOnly
        };
        (version.to_string(), date, header)
    } else {
        let mut parts = rest.splitn(2, " - ");
        let version = parts.next().unwrap_or_default().trim().to_string();
        let date = parts.next().map(|d| d.trim().to_string()).filter(|v| !v.is_empty());
        let header = if date.is_some() {
            HeaderFormat::Plain
        } else {
            HeaderFormat::Custom("## {version}".to_string())
        };
        (version, date, header)
    };

    let version = Version::parse(&version_raw).map_err(|_| ParseIssue {
        line: line_number,
        expected: "a semantic version (x.y.z)".to_string(),
        found: version_raw,
        fix: "replace with a valid version like 1.4.2".to_string(),
    })?;

    Ok((version, date_raw, header))
}

pub fn is_semver_desc_sorted(releases: &[Release]) -> bool {
    releases
        .windows(2)
        .all(|pair| pair[0].version >= pair[1].version)
}

pub fn parse_header_format(input: &str) -> HeaderFormat {
    match input {
        "default" | "brackets" => HeaderFormat::Default,
        "plain" => HeaderFormat::Plain,
        "version-only" => HeaderFormat::VersionOnly,
        other => HeaderFormat::Custom(other.to_string()),
    }
}

fn render_release_header(release: &Release) -> String {
    let version = release.version.to_string();
    let date = release.date.clone().unwrap_or_default();
    match &release.header {
        HeaderFormat::Default => {
            if release.date.is_some() {
                format!("## [{}] - {}", version, date)
            } else {
                format!("## [{}]", version)
            }
        }
        HeaderFormat::Plain => {
            if release.date.is_some() {
                format!("## {} - {}", version, date)
            } else {
                format!("## {}", version)
            }
        }
        HeaderFormat::VersionOnly => format!("## [{}]", version),
        HeaderFormat::Custom(template) => {
            template
                .replace("{version}", &version)
                .replace("{date}", &date)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_renders_roundtrip() {
        let input = "# Changelog\n\n## [1.1.0] - 2026-01-01\n\n### Added\n- a\n\n## [1.0.0] - 2025-12-01\n\n### Fixed\n- b\n";

        let doc = ChangelogDocument::parse(input).expect("parse changelog");
        doc.validate(false).expect("valid changelog");
        let rendered = doc.to_markdown();
        let reparsed = ChangelogDocument::parse(&rendered).expect("reparse markdown");
        assert_eq!(doc, reparsed);
    }

    #[test]
    fn reports_parse_location() {
        let input = "# Changelog\n\n## [1.0.0]\n\n- orphan note\n";
        let err = ChangelogDocument::parse(input).expect_err("should fail");
        assert_eq!(err.line, 5);
        assert!(err.message().contains("Fix:"));
    }

    #[test]
    fn validates_semver_sorting() {
        let doc = ChangelogDocument::parse(
            "# Changelog\n\n## [1.0.0]\n\n### Added\n- a\n\n## [2.0.0]\n\n### Added\n- b\n",
        )
        .expect("parse");
        let err = doc.validate(false).expect_err("should fail ordering");
        assert!(err.message().contains("SemVer"));
    }
}
