use std::process::exit;

use log::{error, info};

use crate::args::Args;

const STRAVA_CANTEEN_DEFAULT: &str = "0000";

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
