use std::process::exit;

use log::error;

use crate::args::Args;

pub fn check_env(args: &Args) {
    if args.discord_webhook_url.is_none() {
        error!("Missing webhook URL for Discord (DISCORD_WEBHOOK_URL)");
        exit(1)
    }
}
