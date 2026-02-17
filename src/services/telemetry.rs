use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub async fn send_event(enabled: bool, event: &str) -> Result<()> {
    if !enabled {
        return Ok(());
    }

    let client = Client::new();
    client.post("https://telemetry.example.com/event")
        .json(&json!({ "event": event }))
        .send()
        .await?;

    Ok(())
}

// [telemetry]
// enabled = true
