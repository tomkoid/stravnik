use log::info;

use crate::args::Args;

const STRAVA_CANTEEN_DEFAULT: &str = "0000";

pub fn check_env(args: &mut Args) {
    if args.strava_canteen.is_none() {
        info!(
            "Strava canteen id not specified (--strava-canteen), defaulting to {}",
            STRAVA_CANTEEN_DEFAULT
        );

        args.strava_canteen = Some(STRAVA_CANTEEN_DEFAULT.to_string());
    }
}
