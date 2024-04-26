mod credentials;
mod meals;

use credentials::Credentials;
use log::info;
use matrix_sdk::{
    config::SyncSettings, matrix_auth::MatrixSession, ruma::RoomId, Client, RoomState,
};

// async fn on_stripped_state_member(
//     room_member: StrippedRoomMemberEvent,
//     client: Client,
//     room: Room,
// ) {
//     if room_member.state_key != client.user_id().unwrap() {
//         return;
//     }
//
//     tokio::spawn(async move {
//         println!("Autojoining room {}", room.room_id());
//
//         let mut delay = 2;
//
//         while let Err(e) = room.join().await {
//             eprintln!("Error joining room: {}", e);
//
//             sleep(Duration::from_secs(delay)).await;
//             delay *= 2;
//
//             if delay > 3600 {
//                 eprintln!("Aborting");
//                 break;
//             }
//         }
//
//         println!("Joined room {}", room.room_id());
//     });
// }

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

    // client.add_event_handler(on_room_message);

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
        // println!("Error: {}", content.as_ref().unwrap_err());
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

    // let settings = SyncSettings::default().token("syt_c3RyYXZuaWtib3Q_llklsSAlrQPGqKKUnSPY_0merLB");

    // let settings = SyncSettings::default().token(sync_token);
    // final sync
    client.sync_once(SyncSettings::default()).await?;
    //
    Ok(())
}

// async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
//     if room.state() != RoomState::Joined {
//         return;
//     }
//
//     let MessageType::Text(text_content) = event.content.msgtype else {
//         return;
//     };
//
//     if text_content.body == "!join" {
//         // let content = RoomMessageEventContent::text_plain("lol");
//         let content = meals::get_meal_message_content().await;
//
//         if content.is_err() {
//             println!("Error: {}", content.unwrap_err());
//             return;
//         }
//
//         room.send(content.unwrap()).await.unwrap();
//
//         println!("Sent message to room {}", room.room_id());
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let credentials = credentials::init_credentials()?;

    login_and_sync(credentials).await?;

    Ok(())
}
