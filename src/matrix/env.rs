use std::process::exit;

use log::{error, info};

use crate::args::Args;

const MATRIX_HOMESERVER_DEFAULT: &str = "https://matrix.org";

pub fn check_env(args: &mut Args) {
    if args.matrix_homeserver.is_none() {
        info!(
            "No homeserver specified via flag, defaulting to default: {}",
            MATRIX_HOMESERVER_DEFAULT
        );
        args.matrix_homeserver = Some(MATRIX_HOMESERVER_DEFAULT.to_string());
    }

    if args.matrix_username.is_none() || args.matrix_password.is_none() {
        error!("Missing username (--matrix-username) or password (--matrix-password)");
        exit(1);
    }

    if args.matrix_room.is_none() {
        error!("Missing matrix room id (--matrix-room)");
        exit(1);
    }
}
