use changelog_gen::core::changelog::ChangelogDocument;
use changelog_gen::core::notes::{dedupe_grouped_notes, should_ignore_commit};
use std::collections::BTreeMap;

#[test]
fn semver_sort_validation_fails_on_lexicographic_order() {
    let input = "# Changelog\n\n## [1.9.0]\n\n### Added\n- a\n\n## [1.10.0]\n\n### Added\n- b\n";
    let doc = ChangelogDocument::parse(input).expect("parse changelog");
    let err = doc.validate(false).expect_err("must fail semver order");
    assert!(err.message().contains("SemVer"));
}

#[test]
fn dedupe_algorithm_is_deterministic() {
    let mut grouped = BTreeMap::new();
    grouped.insert(
        "Fixed".to_string(),
        vec![
            "fix race in parser".to_string(),
            "fix  race  in parser".to_string(),
            "Fix race in parser".to_string(),
            "fix another issue".to_string(),
        ],
    );

    let deduped = dedupe_grouped_notes(&grouped);
    assert_eq!(
        deduped.get("Fixed").expect("fixed section"),
        &vec![
            "fix race in parser".to_string(),
            "fix another issue".to_string()
        ]
    );
}

#[test]
fn ignore_markers_are_supported() {
    assert!(should_ignore_commit("feat: x (skip changelog)"));
    assert!(should_ignore_commit("fix: y (ignore changelog)"));
    assert!(should_ignore_commit("docs: z !changelog"));
    assert!(should_ignore_commit("chore: q !log"));
    assert!(!should_ignore_commit("feat: visible change"));
}

#[test]
fn parser_errors_include_fix_hints() {
    let input = "# Changelog\n\n## [1.0.0]\n\n- orphan note\n";
    let err = ChangelogDocument::parse(input).expect_err("invalid changelog");
    assert_eq!(err.line, 5);
    assert!(err.message().contains("Fix:"));
}
