use log::{error, info};
use reqwest::Client;
use serde_json::json;

pub async fn send_discord_message(
    webhook_url: String,
    payload: serde_json::Value,
) -> Result<(), reqwest::Error> {
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
