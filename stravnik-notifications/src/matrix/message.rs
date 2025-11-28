use log::info;
use matrix_sdk::{Client, RoomState, ruma::RoomId};

use crate::matrix::fmt;

pub async fn send_meal_data(
    client: &Client,
    room_id: &str,
    meal_data: String,
) -> anyhow::Result<()> {
    let room = client.get_room(<&RoomId>::try_from(room_id).unwrap());

    let room = if let Some(room) = room {
        room
    } else {
        return Err(anyhow::anyhow!("Room not found"));
    };

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

    let content = fmt::fmt_meal_data_matrix(meal_data);

    info!("Sending message to room {}...", room.room_id());

    let room_send_status = room.send(content).await;

    if room_send_status.is_err() {
        return Err(anyhow::anyhow!(
            "Error sending message: {}",
            room_send_status.unwrap_err()
        ));
    }

    info!("Sent message to room {}", room.room_id());

    Ok(())
}
