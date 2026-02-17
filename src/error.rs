/// Comprehensive error types for changelog-gen
/// 
/// This module defines all error variants that can occur when using changelog-gen,
/// providing clear error messages and error handling capabilities.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for changelog-gen operations
#[derive(Error, Debug)]
pub enum ChangelogError {
    /// Git operations failed
    #[error("Git operation failed: {0}")]
    GitError(String),

    /// Repository not found
    #[error("Git repository not found at: {}", .0.display())]
    RepositoryNotFound(PathBuf),

    /// Failed to parse commit message
    #[error("Failed to parse commit message: {0}")]
    CommitParseError(String),

    /// Failed to parse semantic version
    #[error("Invalid semantic version: {0}. Expected format: major.minor.patch")]
    VersionParseError(String),

    /// Configuration file not found
    #[error("Configuration file not found: {}", .0.display())]
    ConfigNotFound(PathBuf),

    /// Configuration parsing error
    #[error("Failed to parse configuration file ({format}): {reason}")]
    ConfigParseError { format: String, reason: String },

    /// Template rendering error
    #[error("Template rendering failed: {0}")]
    TemplateError(String),

    /// Plugin loading error
    #[error("Failed to load plugin: {0}")]
    PluginLoadError(String),

    /// GitHub API error
    #[error("GitHub API request failed: {0}")]
    GitHubApiError(String),

    /// Network request failed
    #[error("Network request failed: {0}")]
    NetworkError(String),

    /// File I/O error
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid command line arguments
    #[error("Invalid command line arguments: {0}")]
    InvalidArguments(String),

    /// Template file not found
    #[error("Template file not found: {}", .0.display())]
    TemplateNotFound(PathBuf),

    /// Output file cannot be written
    #[error("Cannot write to output file: {}: {}", .0.display(), .1)]
    OutputWriteError(PathBuf, String),

    /// Regex compilation error
    #[error("Invalid regex pattern: {0}")]
    RegexError(String),

    /// Workspace not found
    #[error("Workspace not found: {0}")]
    WorkspaceNotFound(String),

    /// Unsupported feature
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    /// Telemetry error
    #[error("Telemetry collection failed: {0}")]
    TelemetryError(String),

    /// Generic error for other unexpected situations
    #[error("An unexpected error occurred: {0}")]
    Other(String),
}

impl ChangelogError {
    /// Create a git error
    pub fn git<S: Into<String>>(message: S) -> Self {
        ChangelogError::GitError(message.into())
    }

    /// Create a repository not found error
    pub fn repo_not_found(path: PathBuf) -> Self {
        ChangelogError::RepositoryNotFound(path)
    }

    /// Create a configuration error
    pub fn config_parse_error(format: &str, reason: &str) -> Self {
        ChangelogError::ConfigParseError {
            format: format.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Create a template rendering error
    pub fn template<S: Into<String>>(message: S) -> Self {
        ChangelogError::TemplateError(message.into())
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ChangelogError::ConfigNotFound(_)
                | ChangelogError::TemplateNotFound(_)
                | ChangelogError::UnsupportedFeature(_)
        )
    }

    /// Get the error exit code
    pub fn exit_code(&self) -> i32 {
        match self {
            ChangelogError::InvalidArguments(_) => 1,
            ChangelogError::GitError(_) | ChangelogError::RepositoryNotFound(_) => 2,
            ChangelogError::ConfigNotFound(_) | ChangelogError::ConfigParseError { .. } => 3,
            ChangelogError::TemplateError(_) | ChangelogError::TemplateNotFound(_) => 4,
            ChangelogError::VersionParseError(_) => 5,
            ChangelogError::GitHubApiError(_) | ChangelogError::NetworkError(_) => 6,
            ChangelogError::IoError(_) | ChangelogError::OutputWriteError(_, _) => 7,
            ChangelogError::PluginLoadError(_) => 8,
            ChangelogError::CommitParseError(_) => 9,
            ChangelogError::RegexError(_) => 10,
            ChangelogError::WorkspaceNotFound(_) => 11,
            ChangelogError::TelemetryError(_) => 12,
            ChangelogError::UnsupportedFeature(_) => 13,
            ChangelogError::Other(_) => 99,
        }
    }
}

/// Convenience type alias for Result with ChangelogError
pub type Result<T> = std::result::Result<T, ChangelogError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_exit_codes() {
        let err = ChangelogError::InvalidArguments("test".to_string());
        assert_eq!(err.exit_code(), 1);

        let err = ChangelogError::git("test");
        assert_eq!(err.exit_code(), 2);

        let err = ChangelogError::VersionParseError("test".to_string());
        assert_eq!(err.exit_code(), 5);
    }

    #[test]
    fn test_recoverable_errors() {
        let err = ChangelogError::ConfigNotFound(PathBuf::from("test.toml"));
        assert!(err.is_recoverable());

        let err = ChangelogError::RepositoryNotFound(PathBuf::from("."));
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_error_display() {
        let err = ChangelogError::git("operation failed");
        assert_eq!(err.to_string(), "Git operation failed: operation failed");
    }
}
