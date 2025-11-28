use log::info;
use matrix_sdk::{Client, RoomState, ruma::RoomId};

use crate::{errors::NotificationClientError, matrix::fmt};

pub async fn send_meal_data(
    client: &Client,
    room_id: &str,
    meal_data: String,
) -> Result<(), NotificationClientError> {
    let room = client.get_room(<&RoomId>::try_from(room_id).unwrap());

    let room = if let Some(room) = room {
        room
    } else {
        return Err(NotificationClientError::RoomNotFound(room_id.to_string()));
    };

    // if room is not joined
    if room.state() != RoomState::Joined {
        eprintln!(
            "The bot is not joined to the room ({:?}). Trying to join...",
            room.state()
        );
        room.join().await?;

        println!("Joined to the room! ({:?})", room.state());
    }

    let content = fmt::fmt_meal_data_matrix(meal_data);

    info!("Sending message to room {}...", room.room_id());
    room.send(content).await?;

    info!("Sent message to room {}", room.room_id());

    Ok(())
}
