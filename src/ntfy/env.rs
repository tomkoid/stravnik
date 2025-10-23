use std::process::exit;

use log::{error, info};

use crate::args::Args;

const NTFY_HOST_URL_DEFAULT: &str = "https://ntfy.sh";

pub fn check_env(args: &mut Args) {
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
