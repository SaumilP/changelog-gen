#[cfg(test)]
mod tests {
    use changelog_gen::domain::commit::Commit;

    #[test]
    fn test_commit_creation() {
        // Test commit creation
        let commit = Commit {
            message: "Initial commit".to_string(),
            hash: "abc123".to_string(),
        };

        assert_eq!(commit.message, "Initial commit");
        assert_eq!(commit.hash, "abc123");
    }

    #[test]
    fn test_commit_clone() {
        // Test commit cloning
        let commit = Commit {
            message: "Test commit".to_string(),
            hash: "def456".to_string(),
        };

        let cloned = commit.clone();
        assert_eq!(commit.message, cloned.message);
        assert_eq!(commit.hash, cloned.hash);
    }
}
