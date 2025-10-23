use std::process::exit;

use log::error;

use crate::args::Args;

pub fn check_env(args: &Args) {
    if args.icanteen_url.is_none() {
        error!("Missing iCanteen URL (--icanteen-url)",);
        exit(1)
    }
}
