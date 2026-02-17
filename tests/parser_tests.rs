use changelog_gen::domain::commit::Commit;
use changelog_gen::parser::group_commits;

#[test]
fn test_grouping() {
    let commits = vec![Commit {
        message: "feat: add login".into(),
        hash: "abc".into(),
    }];

    let grouped = group_commits(commits, true).unwrap();
    assert!(grouped.contains_key("feat"));
}
