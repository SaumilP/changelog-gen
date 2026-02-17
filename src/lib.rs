pub mod application;
pub mod bootstrap;
pub mod cli;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod parser;
pub mod traits;

pub use error::{ChangelogError, Result};

pub async fn run() -> crate::Result<()> {
    crate::application::commands::execute().await
}
