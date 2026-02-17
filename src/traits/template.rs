use crate::domain::commit::Commit;
use anyhow::Result;

#[async_trait::async_trait]
pub trait TemplateRenderer: Send + Sync {
    async fn render(&self, commits: Vec<Commit>) -> Result<String>;
}
