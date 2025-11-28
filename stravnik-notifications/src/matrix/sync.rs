use log::info;
use matrix_sdk::{Client, config::SyncSettings, matrix_auth::MatrixSession};

use crate::{errors::NotificationClientError, matrix::credentials::MatrixCredentials};

pub async fn login_and_sync(
    credentials: MatrixCredentials,
) -> Result<Client, NotificationClientError> {
    let client = Client::builder()
        .homeserver_url(credentials.homeserver)
        .build()
        .await?;

    let auth = client.matrix_auth();

    // if the access token is found in the file, use it
    if let Ok(save) = std::fs::read_to_string("save.json") {
        // let session: MatrixSession = serde_json::from_str(&save)?;
        let session: MatrixSession = serde_json::from_str(&save)?;

        client.restore_session(session).await?;
    } else {
        auth.login_username(&credentials.username, &credentials.password)
            .initial_device_display_name("bot")
            .send()
            .await?;
    }

    info!("auth: Logged in as {}", client.user_id().unwrap());
    info!("auth: Access token: {}", client.access_token().unwrap());

    // write access token to file
    if auth.session().is_some() {
        // convert response to json
        let response = serde_json::to_string(&auth.session().unwrap())?;

        std::fs::write("save.json", response)?;
    }

    client_sync(&client).await?;
    Ok(client)
}

pub async fn client_sync(client: &Client) -> Result<(), matrix_sdk::Error> {
    info!("sync: syncing client...");
    client.sync_once(SyncSettings::default()).await?;
    info!("sync: sync done...");

    Ok(())
}
