use std::process::exit;

use log::error;

pub fn init_env() {
    let ic_url = std::env::var("ICANTEEN_URL").unwrap_or_default();

    if ic_url.is_empty() {
        error!("Missing iCanteen URL (ICANTEEN_URL)",);
        exit(1)
    }
}
