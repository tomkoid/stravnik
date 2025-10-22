use clap::Parser;

use crate::services::{MealListService, NotificationService};

/// Send notifications for meal updates
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Meal list service to use
    #[arg(required = true)]
    pub meal_list_service: MealListService,

    /// Type of notification service to use
    #[arg(required = true)]
    pub service: NotificationService,
}
