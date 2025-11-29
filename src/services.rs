use crate::{
    args::Args,
    env::{icanteen_check_env, ntfy_check_env, strava_check_env},
};

#[cfg(feature = "matrix")]
use crate::env::matrix_check_env;

#[cfg(feature = "discord")]
use crate::env::discord_check_env;

use chrono::{DateTime, Local};
use clap::ValueEnum;
use serde::Serialize;
use stravnik_core::{
    icanteen::client::ICanteenClient, meal_data::MealsList, services::MealListService,
    strava::client::StravaClient,
};
use stravnik_notifications::{formatting::MealsListFormatter, ntfy};

#[cfg(feature = "matrix")]
use stravnik_notifications::matrix::{self, credentials::MatrixCredentials};

#[cfg(feature = "discord")]
use stravnik_notifications::discord;

#[derive(Default, ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NotificationService {
    #[cfg(feature = "matrix")]
    Matrix,

    #[default]
    Ntfy,

    #[cfg(feature = "discord")]
    Discord,
}

pub async fn pick_service(mut args: Args, date: DateTime<Local>) -> eyre::Result<()> {
    let meal_d: MealsList;
    match args.meal_list_service {
        MealListService::Strava => {
            strava_check_env(&mut args); // setup arguments needed for strava

            // create new strava client
            let mut sc = StravaClient::new(args.strava_canteen.clone().unwrap());

            // fetch the correct s5url needed for the meal list API request
            sc.fetch_s5url().await;

            meal_d = sc.get_meal_data(date).await?;
        }
        MealListService::ICanteen => {
            icanteen_check_env(&args); // setup arguments for icanteen

            let mut icc = ICanteenClient::new(args.icanteen_url.clone().unwrap());
            meal_d = icc.get_meals(date).await?;
        }
    }

    match args.service {
        #[cfg(feature = "matrix")]
        NotificationService::Matrix => {
            matrix_check_env(&mut args); // initialize environment variables and error if some are missing

            let credentials = init_matrix_credentials(&args);

            let m_client = matrix::sync::login_and_sync(credentials).await?;
            matrix::message::send_meal_data(
                &m_client,
                &args.matrix_room.unwrap(),
                meal_d.basic_fmt(),
            )
            .await?;
            matrix::sync::client_sync(&m_client).await?; // do a final sync
        }
        NotificationService::Ntfy => {
            ntfy_check_env(&mut args);

            let ntfy_client =
                ntfy::client::NtfyClient::new(args.ntfy_host_url.unwrap(), args.ntfy_room.unwrap());
            ntfy_client.send(meal_d.basic_fmt()).await?;
        }
        #[cfg(feature = "discord")]
        NotificationService::Discord => {
            discord_check_env(&args);

            discord::send::send_discord_message(
                args.discord_webhook_url.unwrap(),
                meal_d.discord_fmt(),
            )
            .await?
        }
    }

    _ = meal_d.source;

    Ok(())
}

#[cfg(feature = "matrix")]
fn init_matrix_credentials(args: &Args) -> MatrixCredentials {
    // get username and password from args
    let homeserver = args.matrix_homeserver.clone().unwrap();
    let username = args.matrix_username.clone().unwrap();
    let password = args.matrix_password.clone().unwrap();

    MatrixCredentials::new(homeserver, username, password)
}
