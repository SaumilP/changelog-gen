use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as ProcessCommand;
use tempfile::tempdir;

fn bin_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_changelog-gen"))
}

fn run_git(dir: &std::path::Path, args: &[&str]) {
    let status = ProcessCommand::new("git")
        .args(args)
        .current_dir(dir)
        .status()
        .expect("run git");
    assert!(status.success(), "git command failed: git {:?}", args);
}

fn commit_file(dir: &std::path::Path, file: &str, content: &str, message: &str) {
    fs::write(dir.join(file), content).expect("write file");
    run_git(dir, &["add", "."]);
    run_git(dir, &["commit", "-m", message]);
}

#[test]
fn release_generate_show_and_remove_flow() {
    let dir = tempdir().expect("tempdir");
    run_git(dir.path(), &["init"]);
    run_git(dir.path(), &["config", "user.name", "Test User"]);
    run_git(dir.path(), &["config", "user.email", "test@example.com"]);

    commit_file(dir.path(), "a.txt", "a1", "feat: initial feature");
    run_git(dir.path(), &["tag", "v0.1.0"]);
    commit_file(dir.path(), "b.txt", "b1", "fix: resolve bug");
    commit_file(dir.path(), "c.txt", "c1", "fix: resolve bug");
    commit_file(
        dir.path(),
        "d.txt",
        "d1",
        "chore: tooling cleanup !changelog",
    );

    bin_cmd()
        .current_dir(dir.path())
        .args([
            "release",
            "--bump",
            "patch",
            "--file",
            "CHANGELOG.md",
            "--header",
            "plain",
        ])
        .assert()
        .success();

    let changelog = fs::read_to_string(dir.path().join("CHANGELOG.md")).expect("read changelog");
    assert!(changelog.contains("## 0.0.1 -"));
    assert!(changelog.contains("### Fixed"));
    assert_eq!(changelog.matches("- resolve bug").count(), 1);

    commit_file(dir.path(), "e.txt", "e1", "feat: add api");

    bin_cmd()
        .current_dir(dir.path())
        .args([
            "release",
            "--version",
            "0.2.0",
            "--file",
            "CHANGELOG.md",
            "--header",
            "default",
        ])
        .assert()
        .success();

    let changelog_v2 = fs::read_to_string(dir.path().join("CHANGELOG.md")).expect("read changelog");
    assert!(changelog_v2.contains("## [0.2.0] -"));

    bin_cmd()
        .current_dir(dir.path())
        .args([
            "show",
            "--file",
            "CHANGELOG.md",
            "--range",
            "0.0.1..0.2.0",
            "--converge",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("## [converged]"))
        .stdout(predicate::str::contains("resolve bug"))
        .stdout(predicate::str::contains("add api"));

    fs::write(
        dir.path().join("map.toml"),
        "[types]\nfix = \"Bug Fixes\"\nfeat = \"Features\"\n",
    )
    .expect("write map");

    bin_cmd()
        .current_dir(dir.path())
        .args([
            "generate", "--since", "v0.1.0", "--until", "HEAD", "--map", "map.toml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("### Bug Fixes"))
        .stdout(predicate::str::contains("resolve bug"));

    bin_cmd()
        .current_dir(dir.path())
        .args(["generate", "--milestone", "Q1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--milestone requires --github"));

    bin_cmd()
        .current_dir(dir.path())
        .args([
            "remove",
            "--version",
            "0.0.1",
            "--file",
            "CHANGELOG.md",
            "--yes",
        ])
        .assert()
        .success();

    let after_remove = fs::read_to_string(dir.path().join("CHANGELOG.md")).expect("read changelog");
    assert!(!after_remove.contains("0.0.1"));
    assert!(after_remove.contains("0.2.0"));
}
