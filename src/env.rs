use std::process::exit;

use log::{error, info};

use crate::args::Args;

const STRAVA_CANTEEN_DEFAULT: &str = "0000";
const NTFY_HOST_URL_DEFAULT: &str = "https://ntfy.sh";
#[cfg(feature = "matrix")]
const MATRIX_HOMESERVER_DEFAULT: &str = "https://matrix.org";

pub fn icanteen_check_env(args: &Args) {
    if args.icanteen_url.is_none() {
        error!("Missing iCanteen URL (--icanteen-url)",);
        exit(1)
    }
}

pub fn strava_check_env(args: &mut Args) {
    if args.strava_canteen.is_none() {
        info!(
            "Strava canteen id not specified (--strava-canteen), defaulting to {}",
            STRAVA_CANTEEN_DEFAULT
        );

        args.strava_canteen = Some(STRAVA_CANTEEN_DEFAULT.to_string());
    }
}

#[cfg(feature = "discord")]
pub fn discord_check_env(args: &Args) {
    if args.discord_webhook_url.is_none() {
        error!("Missing webhook URL for Discord (DISCORD_WEBHOOK_URL)");
        exit(1)
    }
}

#[cfg(feature = "matrix")]
pub fn matrix_check_env(args: &mut Args) {
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

pub fn ntfy_check_env(args: &mut Args) {
    if args.ntfy_host_url.is_none() {
        info!(
            "No host url specified, defaulting to default: {}",
            NTFY_HOST_URL_DEFAULT
        );

        args.ntfy_host_url = Some(NTFY_HOST_URL_DEFAULT.to_string());
    }

    if args.ntfy_room.is_none() {
        error!("No Ntfy room specified");
        exit(1);
    }
}
