use crate::domain::commit::Commit;
use anyhow::Result;

#[async_trait::async_trait]
pub trait GitRepository: Send + Sync {
    async fn get_commits(&self) -> Result<Vec<Commit>>;
}
