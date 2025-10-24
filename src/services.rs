use crate::{args, meal_data::MealsList};
use clap::ValueEnum;
use serde::Serialize;

use crate::{
    credentials, discord,
    icanteen::{self, client::ICanteenClient},
    matrix, ntfy, services,
    strava::{self, client::StravaClient},
};

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

    ICanteen,
}

pub async fn pick_service(mut args: args::Args) -> anyhow::Result<()> {
    let meal_d: MealsList;
    match args.meal_list_service {
        services::MealListService::Strava => {
            strava::env::check_env(&mut args); // setup arguments needed for strava

            // create new strava client
            let mut sc = StravaClient::new(args.strava_canteen.clone().unwrap());

            // fetch the correct s5url needed for the meal list API request
            sc.fetch_s5url().await;

            meal_d = sc.get_meal_data().await?;
        }
        services::MealListService::ICanteen => {
            icanteen::env::check_env(&args); // setup arguments for icanteen

            let icc = ICanteenClient::new(args.icanteen_url.clone().unwrap());
            meal_d = icc.get_meals().await?;
        }
    }

    match args.service {
        services::NotificationService::Matrix => {
            matrix::env::check_env(&mut args); // initialize environment variables and error if some are missing

            let credentials = credentials::init_matrix_credentials(&args);

            if credentials.is_err() {
                return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
            }

            let m_client = matrix::sync::login_and_sync(credentials?).await?;
            matrix::message::send_meal_data(
                &m_client,
                &args.matrix_room.unwrap(),
                meal_d.basic_fmt(),
            )
            .await?;
            matrix::sync::client_sync(&m_client).await?; // do a final sync
        }
        services::NotificationService::Ntfy => {
            ntfy::env::check_env(&mut args);

            let ntfy_client =
                ntfy::client::NtfyClient::new(args.ntfy_host_url.unwrap(), args.ntfy_room.unwrap());
            ntfy_client.send(meal_d.basic_fmt()).await?;
        }
        services::NotificationService::Discord => {
            discord::env::check_env(&args);

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
