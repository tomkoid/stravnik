use chrono::DateTime;
use chrono::Local;
use log::debug;
use log::info;
use serde::Serialize;

use crate::errors::MealClientError;
use crate::meal_data::Meal;
use crate::meal_data::MealsList;
use crate::services::MealListService;
use crate::strava::client::StravaClient;
use crate::utils::ToDateStringExt;

#[derive(Serialize)]
struct RequestPayload {
    cislo: String,
    #[serde(rename = "ignoreCert")]
    ignore_cert: bool,
    lang: String,
    s5url: String,
}

impl StravaClient {
    pub async fn get_meal_data(&self, date: DateTime<Local>) -> Result<MealsList, MealClientError> {
        // convert date to string in format DD.MM.YYYY
        let date = date.to_date_string();

        info!("strava: getting meals from API...");

        // get meals from API
        let meals = self.get_meals().await?;

        info!("strava: got meals!");

        let mut today_meals: serde_json::Value = serde_json::Value::Null;

        let meals_array = if let Some(ma) = meals.as_array() {
            ma
        } else {
            return Err(MealClientError::ParseError(format!(
                "Failed to parse meals, response from Strava API invalid: {}",
                meals,
            )));
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
                    debug!("found meal table: {}", meal);
                    break;
                };
            }
        }

        // no meal found
        if today_meals == serde_json::Value::Null {
            return Err(MealClientError::MealNotFound);
        }

        debug!("MEALS len: {}", today_meals.as_array().unwrap().len());

        let mut found_meals: Vec<Meal> = Vec::new();

        'outer: for meal in today_meals.as_array().unwrap() {
            let meal_description = meal["nazev"].as_str().unwrap().to_string();

            // if meal name is already in the list, skip it
            for found_meal in &found_meals {
                if found_meal.name == meal_description {
                    debug!("skipping meal, duplicate found: {}", meal_description);
                    continue 'outer;
                }
            }

            let meal_date = today_meals[0]["datum"].as_str().unwrap().to_string();
            let meal_course = meal["druh_chod"].as_str().unwrap().to_string();

            found_meals.push(Meal {
                name: String::new(),
                description: meal_description,
                date: meal_date,
                course: meal_course,
            });
        }

        Ok(MealsList::new(found_meals, MealListService::Strava))
    }

    async fn get_meals(&self) -> Result<serde_json::Value, MealClientError> {
        if self.s5url.is_none() {
            return Err(MealClientError::InvalidConfig(
                "s5url is not set, first fetch it with client.fetch_s5url()".to_string(),
            ));
        }

        let payload = RequestPayload {
            cislo: self.canteen_id.clone(),
            ignore_cert: false,
            lang: "EN".to_string(),
            s5url: self.s5url.clone().unwrap(),
        };

        debug!(
            "strava: payload: {}",
            serde_json::to_string(&payload).unwrap()
        );

        let request = self
            .get_client()
            .post("https://app.strava.cz/api/jidelnicky")
            .header("Content-Type", "text/plain")
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?;

        let json: serde_json::Value = serde_json::from_str(&request.text().await?).unwrap();

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_meals_without_s5url() {
        let client = StravaClient::new("12345".to_string());
        let result = client.get_meals().await;
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, MealClientError::InvalidConfig(_)));
        assert_eq!(
            err.to_string(),
            "Invalid config: s5url is not set, first fetch it with client.fetch_s5url()"
        );
    }
}
