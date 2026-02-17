use anyhow::Result;

pub async fn run(repo: &str) -> Result<()> {
    let url = format!(
        "https://api.github.com/repos/{repo}/releases/latest"
    );

    let res: serde_json::Value = reqwest::get(&url).await?.json().await?;
    println!("Latest version: {}", res["tag_name"]);

    Ok(())
}
