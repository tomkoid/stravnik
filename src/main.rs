mod credentials;
mod env;
mod meals;

use credentials::Credentials;
use env::init_env;
use log::info;
use matrix_sdk::{
    config::SyncSettings, matrix_auth::MatrixSession, ruma::RoomId, Client, RoomState,
};

async fn login_and_sync(credentials: Credentials) -> anyhow::Result<()> {
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

    info!("sync: Syncing...");
    client.sync_once(SyncSettings::default()).await.unwrap();

    info!("sync: Sync done!");

    let room_string = std::env::var("MATRIX_ROOM").expect("Missing MATRIX_ROOM");

    let room = client
        .get_room(<&RoomId>::try_from(room_string.as_str()).unwrap())
        .unwrap();

    // if room is not joined
    if room.state() != RoomState::Joined {
        eprintln!(
            "The bot is not joined to the room ({:?}). Trying to join...",
            room.state()
        );
        let result = room.join().await;
        if result.is_err() {
            return Err(anyhow::anyhow!(
                "Error joining room: {}",
                result.unwrap_err()
            ));
        }

        println!("Joined to the room! ({:?})", room.state());
    }

    let content = meals::get_meal_message_content().await;

    if content.is_err() {
        return Err(anyhow::anyhow!("{}", content.unwrap_err()));
    }

    let room_send_status = room.send(content.unwrap()).await;

    if room_send_status.is_err() {
        return Err(anyhow::anyhow!(
            "Error sending message: {}",
            room_send_status.unwrap_err()
        ));
    }

    info!("Sent message to room {}", room.room_id());

    // final sync
    client.sync_once(SyncSettings::default()).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    init_env(); // initialize environment variables and error if some are missing

    let credentials = credentials::init_credentials();

    if credentials.is_err() {
        return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
    }

    login_and_sync(credentials?).await?;

    Ok(())
}
