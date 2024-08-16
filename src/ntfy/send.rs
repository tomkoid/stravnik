use anyhow::anyhow;
use log::info;

pub async fn send_notification(text: String) -> anyhow::Result<()> {
    let host_url = std::env::var("NTFY_HOST_URL").unwrap();
    let room = std::env::var("NTFY_ROOM").unwrap();

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/{}", host_url, room))
        .header("Content-Type", "text/plain")
        .body(text.clone())
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(response.text().await?));
    }

    info!("Sent notification! URL: {}, ROOM: {}", host_url, room);

    Ok(())
}
