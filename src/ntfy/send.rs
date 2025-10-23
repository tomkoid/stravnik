use anyhow::anyhow;
use log::info;

use crate::ntfy::client::NtfyClient;

impl NtfyClient {
    pub async fn send(&self, text: String) -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}/{}", self.host_url, self.room))
            .header("Content-Type", "text/plain")
            .body(text.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(response.text().await?));
        }

        info!(
            "ntfy: sent notification! URL: {}, ROOM: {}",
            self.host_url, self.room
        );

        Ok(())
    }
}
