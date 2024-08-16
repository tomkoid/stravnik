use std::process::exit;

use log::{error, info};

const MATRIX_HOMESERVER_DEFAULT: &str = "https://matrix.org";

pub fn init_env() {
    let homeserver = std::env::var("MATRIX_HOMESERVER").unwrap_or(String::new());
    let username = std::env::var("MATRIX_USERNAME").unwrap_or(String::new());
    let password = std::env::var("MATRIX_PASSWORD").unwrap_or(String::new());
    let room = std::env::var("MATRIX_ROOM").unwrap_or(String::new());

    if homeserver.is_empty() {
        info!(
            "No homeserver specified, defaulting to default: {}",
            MATRIX_HOMESERVER_DEFAULT
        );
        std::env::set_var("MATRIX_HOMESERVER", MATRIX_HOMESERVER_DEFAULT);
    }

    if username.is_empty() || password.is_empty() {
        error!("Missing username (MATRIX_USERNAME) or password (MATRIX_PASSWORD)");
        exit(1);
    }

    if room.is_empty() {
        error!("Missing matrix room id (MATRIX_ROOM)");
        exit(1);
    }
}
