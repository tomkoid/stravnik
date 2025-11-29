use clap::Parser;
use stravnik_core::services::MealListService;

use crate::services::NotificationService;

/// Send notifications for meal updates
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    /// Meal list service to use
    pub meal_list_service: MealListService,

    /// Type of notification service to use
    #[arg(required = true)]
    pub service: NotificationService,

    // ----- Meal list service configs -----
    #[arg(long)]
    pub strava_canteen: Option<String>,

    #[arg(long)]
    pub icanteen_url: Option<String>,

    // ----- Matrix configs -----
    #[cfg(feature = "matrix")]
    #[arg(long)]
    pub matrix_homeserver: Option<String>,

    #[cfg(feature = "matrix")]
    #[arg(long)]
    pub matrix_username: Option<String>,

    #[cfg(feature = "matrix")]
    #[arg(long)]
    pub matrix_password: Option<String>,

    #[cfg(feature = "matrix")]
    #[arg(long)]
    pub matrix_room: Option<String>,

    // ----- Ntfy configs -----
    #[arg(long)]
    pub ntfy_host_url: Option<String>,

    #[arg(long)]
    pub ntfy_room: Option<String>,

    // ----- Discord configs -----
    #[arg(long)]
    #[cfg(feature = "discord")]
    pub discord_webhook_url: Option<String>,
}
