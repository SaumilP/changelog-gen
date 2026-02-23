use crate::domain::commit::Commit;
use std::collections::{BTreeMap, BTreeSet};

pub const IGNORE_MARKERS: [&str; 4] = [
    "(skip changelog)",
    "(ignore changelog)",
    "!changelog",
    "!log",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub section: String,
    pub text: String,
}

pub fn should_ignore_commit(message: &str) -> bool {
    let lower = message.to_lowercase();
    IGNORE_MARKERS.iter().any(|marker| lower.contains(marker))
}

pub fn parse_conventional_commit(message: &str) -> Option<(String, String)> {
    let trimmed = message.trim();
    let (head, tail) = trimmed.split_once(':')?;
    let description = tail.trim();
    if description.is_empty() {
        return None;
    }

    let kind = head.split('(').next().unwrap_or(head).trim();
    let kind = kind.trim_end_matches('!');

    if kind.is_empty() {
        return None;
    }

    Some((kind.to_string(), description.to_string()))
}

pub fn map_type_to_section(kind: &str, mapping: &BTreeMap<String, String>) -> String {
    if let Some(section) = mapping.get(kind) {
        return section.clone();
    }

    match kind {
        "feat" => "Added".to_string(),
        "fix" => "Fixed".to_string(),
        "perf" | "refactor" => "Changed".to_string(),
        "docs" => "Documentation".to_string(),
        "test" | "chore" | "build" | "ci" => "Maintenance".to_string(),
        _ => "Other".to_string(),
    }
}

pub fn notes_from_commits(
    commits: &[Commit],
    mapping: &BTreeMap<String, String>,
) -> BTreeMap<String, Vec<String>> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for commit in commits {
        if should_ignore_commit(&commit.message) {
            continue;
        }

        let text = commit
            .message
            .lines()
            .next()
            .map(str::trim)
            .unwrap_or("")
            .to_string();
        if text.is_empty() {
            continue;
        }

        if let Some((kind, description)) = parse_conventional_commit(&text) {
            let section = map_type_to_section(&kind, mapping);
            grouped.entry(section).or_default().push(description);
        } else {
            grouped.entry("Other".to_string()).or_default().push(text);
        }
    }

    dedupe_grouped_notes(&grouped)
}

pub fn dedupe_grouped_notes(
    grouped: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>> {
    let mut deduped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (section, notes) in grouped {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut list = Vec::new();

        for note in notes {
            let key = canonical_note_key(note);
            if seen.insert(key) {
                list.push(note.clone());
            }
        }

        if !list.is_empty() {
            deduped.insert(section.clone(), list);
        }
    }

    deduped
}

pub fn canonical_note_key(note: &str) -> String {
    note.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_marker_commits() {
        assert!(should_ignore_commit("feat: x (skip changelog)"));
        assert!(should_ignore_commit("fix: y !changelog"));
        assert!(!should_ignore_commit("feat: real change"));
    }

    #[test]
    fn dedupes_notes_deterministically() {
        let mut grouped = BTreeMap::new();
        grouped.insert(
            "Added".to_string(),
            vec![
                "new api".to_string(),
                "new   api".to_string(),
                "NEW api".to_string(),
                "another item".to_string(),
            ],
        );

        let deduped = dedupe_grouped_notes(&grouped);
        assert_eq!(
            deduped.get("Added").expect("section"),
            &vec!["new api".to_string(), "another item".to_string()]
        );
    }
}
