use log::info;

const STRAVA_CANTEEN_DEFAULT: &str = "0000";

pub fn init_env() {
    let strava_canteen = std::env::var("STRAVA_CANTEEN").unwrap_or_default();

    if strava_canteen.is_empty() {
        info!(
            "Missing strava canteen id (STRAVA_CANTEEN), defaulting to {}",
            STRAVA_CANTEEN_DEFAULT
        );
        std::env::set_var("STRAVA_CANTEEN", STRAVA_CANTEEN_DEFAULT);
    }
}
