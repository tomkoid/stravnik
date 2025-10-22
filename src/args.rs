use clap::Parser;

use crate::services::NotificationService;

/// Send notifications for meal updates
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Type of notification service to use
    #[arg(required = true)]
    pub service: NotificationService,
}
