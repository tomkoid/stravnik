use anyhow::anyhow;
use chrono::{Datelike, Local};
use log::{debug, info};
use serde::Serialize;

#[derive(Serialize)]
struct RequestPayload {
    cislo: String,
    #[serde(rename = "ignoreCert")]
    ignore_cert: bool,
    lang: String,
    s5url: String,
}

#[derive(Serialize, Debug)]
struct S4PolozkyRequest {
    cislo: String,
    lang: String,
    polozky: String,
}

pub struct StravaClient {
    pub canteen_id: String,
    pub s5url: Option<String>,

    client: reqwest::Client,
}

impl StravaClient {
    pub fn new() -> Self {
        let canteen_id = std::env::var("STRAVA_CANTEEN").unwrap();
        let client = reqwest::Client::new();

        Self {
            canteen_id,
            s5url: None,
            client,
        }
    }

    pub async fn fetch_s5url(&mut self) {
        let req_payload = S4PolozkyRequest {
            cislo: self.canteen_id.clone(),
            lang: "EN".to_string(),
            polozky: "URLWSDL_S-URL".to_string(),
        };
        debug!("S4 Polozky request payload: {:?}", req_payload);
        let request = self
            .client
            .post("https://app.strava.cz/api/s4Polozky")
            .header("Content-Type", "text/plain;charset=UTF-8")
            .body(serde_json::to_string(&req_payload).unwrap())
            .send()
            .await
            .unwrap();

        let json: serde_json::Value = serde_json::from_str(&request.text().await.unwrap()).unwrap();

        self.s5url = Some(json["urlwsdl_s"][0].as_str().unwrap().to_string());
    }

    pub async fn get_meal_data(&self) -> anyhow::Result<String> {
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
        let meals = self.get_meals().await?;

        info!("Got meals!");

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
            let table_obj = match table.as_object() {
                Some(o) => o,
                None => {
                    debug!("failed to parse this table: {}", table);
                    continue;
                }
            };

            for (_, meal) in table_obj {
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
            let druh_chod = meal["druh_chod"].as_str().unwrap().trim();
            let druh_chod_string = format!("[{}]", druh_chod);

            text = format!(
                "{}{}. *{}* {}\n",
                text,
                index,
                druh_chod_string,
                meal["nazev"].as_str().unwrap()
            );
            index += 1;
        }

        Ok(text.trim().to_string())
    }

    async fn get_meals(&self) -> anyhow::Result<serde_json::Value> {
        if self.s5url.is_none() {
            return Err(anyhow!(
                "s5url is not set, first fetch it with client.fetch_s5url()"
            ));
        }

        let payload = RequestPayload {
            cislo: self.canteen_id.clone(),
            // cislo: ,
            ignore_cert: false,
            lang: "EN".to_string(),
            s5url: self.s5url.clone().unwrap(),
        };

        debug!("Payload: {}", serde_json::to_string(&payload).unwrap());

        let request = self
            .client
            .post("https://app.strava.cz/api/jidelnicky")
            .header("Content-Type", "text/plain")
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?;

        let json: serde_json::Value = serde_json::from_str(&request.text().await?).unwrap();

        Ok(json)
    }
}
