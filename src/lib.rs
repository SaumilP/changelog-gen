pub mod bootstrap;
pub mod cli;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod traits;
pub mod config;
pub mod parser;
pub mod error;

pub use error::{ChangelogError, Result};

pub async fn run() -> crate::Result<()> {
    crate::application::commands::execute().await
}
