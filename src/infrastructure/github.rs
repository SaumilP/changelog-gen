use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize)]
pub struct PullRequest {
    pub title: String,
    pub html_url: String,
}

pub async fn fetch_pr(repo: &str, number: u32) -> Result<PullRequest> {
    let client = Client::new();

    let url = format!(
        "https://api.github.com/repos/{}/pulls/{}",
        repo, number
    );

    let res = client
        .get(&url)
        .header("User-Agent", "changelog-gen")
        .send()
        .await?
        .json::<PullRequest>()
        .await?;

    Ok(res)
}

pub fn generate_compare_link() -> Result<String> {
    // TODO: Implement GitHub compare link generation
    Ok(String::new())
}
