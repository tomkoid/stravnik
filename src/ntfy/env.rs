use std::process::exit;

use log::{error, info};

const NTFY_HOST_URL_DEFAULT: &str = "https://ntfy.sh";

pub fn init_env() {
    let host_url = std::env::var("NTFY_HOST_URL").unwrap_or_default();
    let room = std::env::var("NTFY_ROOM").unwrap_or_default();

    if host_url.is_empty() {
        info!(
            "No host url specified, defaulting to default: {}",
            NTFY_HOST_URL_DEFAULT
        );
        std::env::set_var("NTFY_HOST_URL", NTFY_HOST_URL_DEFAULT);
    }

    if room.is_empty() {
        error!("No Ntfy room specified");
        exit(1);
    }
}
