use changelog_gen::core::changelog::ChangelogDocument;

#[test]
fn fixture_roundtrip_is_stable() {
    let fixture = include_str!("fixtures/changelog_roundtrip.md");
    let parsed = ChangelogDocument::parse(fixture).expect("parse fixture");
    parsed.validate(true).expect("validate fixture");
    let rendered = parsed.to_markdown();
    let reparsed = ChangelogDocument::parse(&rendered).expect("reparse");
    assert_eq!(parsed, reparsed);
}
