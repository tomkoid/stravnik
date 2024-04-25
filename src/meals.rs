use anyhow::anyhow;
use chrono::{Datelike, Local};
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
        cislo: std::env::var("STRAVA_CANTEEN").unwrap_or("0000".to_string()),
        ignore_cert: false,
        lang: "EN".to_string(),
        s5url: "".to_string(),
    };

    let request = client
        .post("https://app.strava.cz/api/jidelnicky")
        .header("Content-Type", "text/plain")
        .body(serde_json::to_string(&payload).unwrap())
        .send()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&request.text().await?).unwrap();

    Ok(json)
}

pub async fn get_meal_message_content() -> anyhow::Result<RoomMessageEventContent> {
    // get today's date
    let date_today = Local::now();
    let date = format!(
        "{:02}.{:02}.{}",
        date_today.day(),
        date_today.month(),
        date_today.year()
    );

    // get meals from API
    let meals = get_meals().await;

    if meals.is_err() {
        return Ok(RoomMessageEventContent::text_plain(format!(
            "Error: {:?}",
            meals.unwrap_err()
        )));
    }

    // println!("{}", meals.as_ref().unwrap()[0]["table0"]);

    let meals = &meals.unwrap();
    let mut today_meals: serde_json::Value = serde_json::Value::Null;

    // for every table in the response
    for table in meals.as_array().unwrap() {
        // println!("{:?}", table);
        for (_, meal) in table.as_object().unwrap() {
            if meal[0]["datum"].as_str().unwrap() == date {
                today_meals = meal.clone();
                // println!("{:?}", meal);
                break;
            };
            // if the item has the correct date
            // if item["datum"].as_str().unwrap() == "24.04.2024" {}
        }
    }

    // no meal found
    if today_meals == serde_json::Value::Null {
        return Err(anyhow!("Today's meal not found"));
    }

    // if meal found, format it
    let text = format!(
        "Obědy pro {}\n1. {}\n2. {}\n3. {}",
        today_meals[0]["datum"].as_str().unwrap(),
        today_meals[0]["nazev"].as_str().unwrap(),
        today_meals[1]["nazev"].as_str().unwrap(),
        today_meals[2]["nazev"].as_str().unwrap()
    );

    Ok(RoomMessageEventContent::text_plain(text))
}
