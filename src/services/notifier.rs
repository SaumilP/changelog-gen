use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub async fn notify_slack(webhook: &str, message: &str) -> Result<()> {
    let client = Client::new();
    client.post(webhook)
        .json(&json!({ "text": message }))
        .send()
        .await?;
    Ok(())
}

pub async fn notify_discord(webhook: &str, message: &str) -> Result<()> {
    let client = Client::new();
    client.post(webhook)
        .json(&json!({ "content": message }))
        .send()
        .await?;
    Ok(())
}

// [notifications]
// slack_webhook = "..."
// discord_webhook = "..."
