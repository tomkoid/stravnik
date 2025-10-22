use std::process::exit;

use log::error;

pub fn init_env() {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL").unwrap_or_default();

    if webhook_url.is_empty() {
        error!("Missing webhook URL for Discord (DISCORD_WEBHOOK_URL)");
        exit(1)
    }
}
