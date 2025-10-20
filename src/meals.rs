use anyhow::anyhow;
use chrono::{Datelike, Local};
use log::{debug, info};
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use serde::Serialize;

#[derive(Serialize)]
struct RequestPayload {
    cislo: String,
    #[serde(rename = "ignoreCert")]
    ignore_cert: bool,
    lang: String,
    s5url: String,
}

async fn get_meals() -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();

    let payload = RequestPayload {
        cislo: std::env::var("STRAVA_CANTEEN").unwrap(),
        ignore_cert: false,
        lang: "EN".to_string(),
        s5url: "".to_string(),
    };

    debug!("Payload: {}", serde_json::to_string(&payload).unwrap());

    let request = client
        .post("https://app.strava.cz/api/jidelnicky")
        .header("Content-Type", "text/plain")
        .body(serde_json::to_string(&payload).unwrap())
        .send()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&request.text().await?).unwrap();

    Ok(json)
}

pub async fn get_meal_data() -> anyhow::Result<String> {
    // get today's date
    let date_today = Local::now();
    let date = format!(
        "{:02}.{:02}.{}",
        date_today.day(),
        date_today.month(),
        date_today.year()
    );

    info!("Getting meals from API...");

    // get meals from API
    let meals = get_meals().await;

    info!("Got meals!");

    if meals.is_err() {
        return Ok(format!("Error: {:?}", meals.unwrap_err()));
    }

    let meals = &meals.unwrap();
    let mut today_meals: serde_json::Value = serde_json::Value::Null;

    let meals_array = if let Some(ma) = meals.as_array() {
        ma
    } else {
        return Err(anyhow!(
            "Failed to parse meals, response from Strava API invalid: {}",
            meals
        ));
    };

    // for every table in the response
    for table in meals_array {
        for (_, meal) in table.as_object().unwrap() {
            if meal[0]["datum"].as_str().unwrap() == date {
                today_meals = meal.clone();
                debug!("Found meal: {}", meal);
                break;
            };
        }
    }

    // no meal found
    if today_meals == serde_json::Value::Null {
        return Err(anyhow!("Today's meal not found"));
    }

    debug!("MEALS len: {}", today_meals.as_array().unwrap().len());

    // if meal found, format it
    let mut text = format!(
        "### Obědy pro **{}**:\n",
        today_meals[0]["datum"].as_str().unwrap(),
    );

    let mut index = 1;
    for meal in today_meals.as_array().unwrap() {
        text = format!("{}{}. {}\n", text, index, meal["nazev"].as_str().unwrap());
        index += 1;
    }

    Ok(text)
}

pub fn fmt_meal_data_matrix(text: String) -> RoomMessageEventContent {
    RoomMessageEventContent::text_markdown(text)
}
