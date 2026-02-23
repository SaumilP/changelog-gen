use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn bin_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_changeloggen-cli"))
}

#[test]
fn help_lists_required_subcommands() {
    let mut cmd = bin_cmd();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("validate"))
        .stdout(predicate::str::contains("generate"))
        .stdout(predicate::str::contains("release"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn new_creates_scaffold_and_validate_passes() {
    let dir = tempdir().expect("tempdir");
    let changelog = dir.path().join("CHANGELOG.md");

    bin_cmd()
        .current_dir(dir.path())
        .args(["new", "--file", "CHANGELOG.md"])
        .assert()
        .success();

    let content = fs::read_to_string(&changelog).expect("read changelog");
    assert!(content.contains("# Changelog"));

    bin_cmd()
        .current_dir(dir.path())
        .args(["validate", "--file", "CHANGELOG.md"])
        .assert()
        .success();
}

#[test]
fn validate_fails_with_human_friendly_error() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("CHANGELOG.md"),
        "# Changelog\n\n## [1.0.0]\n\n- orphan\n",
    )
    .expect("write invalid changelog");

    bin_cmd()
        .current_dir(dir.path())
        .args(["validate", "--file", "CHANGELOG.md", "--strict"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("line 5"))
        .stderr(predicate::str::contains("Fix:"));
}

#[test]
fn remove_requires_yes_flag() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("CHANGELOG.md"),
        "# Changelog\n\n## [1.0.0]\n\n### Added\n- first\n",
    )
    .expect("write changelog");

    bin_cmd()
        .current_dir(dir.path())
        .args(["remove", "--version", "1.0.0", "--file", "CHANGELOG.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--yes"));
}

#[test]
fn remove_fails_if_version_missing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("CHANGELOG.md"),
        "# Changelog\n\n## [1.0.0]\n\n### Added\n- first\n",
    )
    .expect("write changelog");

    bin_cmd()
        .current_dir(dir.path())
        .args([
            "remove",
            "--version",
            "2.0.0",
            "--file",
            "CHANGELOG.md",
            "--yes",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("was not found"));
}
