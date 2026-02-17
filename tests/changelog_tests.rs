#[cfg(test)]
mod tests {
    use changelog_gen::domain::changelog;

    #[test]
    fn test_generate_changelog() {
        // Test basic changelog generation
        let grouped = std::collections::BTreeMap::new();
        let result = changelog::generate(grouped, None, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_changelog_with_version() {
        // Test changelog generation with version
        let grouped = std::collections::BTreeMap::new();
        let result = changelog::generate(grouped, Some("1.0.0".to_string()), false);
        assert!(result.is_ok());
    }
}
