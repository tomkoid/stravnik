use clap::ValueEnum;
use serde::Serialize;

#[derive(Default, ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NotificationService {
    Matrix,

    #[default]
    Ntfy,

    Discord,
}

#[derive(Default, ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MealListService {
    #[default]
    Strava,
}
