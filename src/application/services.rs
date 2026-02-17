use crate::traits::git::GitRepository;
use crate::traits::template::TemplateRenderer;
use anyhow::Result;

pub struct GeneratorService<G, T>
where
    G: GitRepository,
    T: TemplateRenderer,
{
    pub git: G,
    pub template: T,
}

impl<G, T> GeneratorService<G, T>
where
    G: GitRepository,
    T: TemplateRenderer,
{
    pub async fn generate(&self) -> Result<String> {
        let commits = self.git.get_commits().await?;

        tracing::info!("Found {} commits", commits.len());

        self.template.render(commits).await
    }
}
