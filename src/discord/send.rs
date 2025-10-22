use log::{error, info};
use reqwest::Client;
use serde_json::json;

pub async fn send_discord_message(payload: serde_json::Value) -> anyhow::Result<()> {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL").unwrap();

    let client = Client::new();

    // message payload (TODO: embeds, username, avatar_url, etc.)
    let payload = json!(payload);

    let response = client.post(&webhook_url).json(&payload).send().await?;

    if response.status().is_success() {
        info!("discord: sent notification! URL: {}", webhook_url);
    } else {
        error!(
            "discord: failed to send message: {}",
            response.text().await?
        );
    }

    Ok(())
}
