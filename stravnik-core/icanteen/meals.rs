use crate::{
    errors::MealClientError,
    icanteen::client::ICanteenClient,
    meal_data::{Meal, MealsList},
    services::MealListService,
};
use chrono::{DateTime, Local};
use log::{debug, info};
use regex::Regex;
use scraper::{Html, Selector};

use crate::utils::ToDateStringExt;

impl ICanteenClient {
    pub async fn get_meals(&mut self, date: DateTime<Local>) -> Result<MealsList, MealClientError> {
        info!("icanteen: getting meals...");

        self.date = Some(date);
        let resp = self.fetch_meals().await?;
        let meals = self.parse_meals_response(resp);

        info!("icanteen: got meals!");

        Ok(MealsList::new(meals, MealListService::ICanteen))
    }

    fn parse_meals_response(&self, meals_resp: String) -> Vec<Meal> {
        let mut meals: Vec<Meal> = Vec::new();

        let document = Html::parse_document(&meals_resp);

        if self.date.is_none() {
            debug!("icanteen: date is none, using today as date");
        }

        let today = self.date.unwrap_or_else(Local::now);
        let today_string = today.to_date_string();

        let target_day_id = format!("day-{}", today.format("%Y-%m-%d"));

        let re_space = Regex::new(r"\s{2,}").unwrap();

        let day_sel = Selector::parse("div.jidelnicekDen").unwrap();
        let date_sel = Selector::parse("div.jidelnicekTop.semibold").unwrap();
        let container_sel = Selector::parse("div.container").unwrap();
        let meal_name_sel = Selector::parse("div.smallBoldTitle span").unwrap();
        let meal_course_sel =
            Selector::parse("div.shrinkedColumn.jidelnicekItem span[style*='green']").unwrap();
        let meal_desc_sel = Selector::parse("div.column.jidelnicekItem").unwrap();

        for day in document.select(&day_sel) {
            let date_div = day.select(&date_sel).next();

            if let Some(date_el) = date_div {
                if let Some(id) = date_el.value().attr("id") {
                    if id == target_day_id {
                        let date = date_el
                            .text()
                            .collect::<String>()
                            .replace('\u{00a0}', " ")
                            .trim()
                            .to_string();

                        // println!("Found date: {date}");
                        'outer: for container in day.select(&container_sel) {
                            let name = container
                                .select(&meal_name_sel)
                                .next()
                                .map(|e| e.text().collect::<String>().trim().to_string())
                                .unwrap_or_default();

                            let course = container
                                .select(&meal_course_sel)
                                .next()
                                .map(|e| e.text().collect::<String>().trim().to_string())
                                .unwrap_or_default();

                            let description = container
                                .select(&meal_desc_sel)
                                .next()
                                .map(|e| {
                                    re_space
                                        .replace_all(
                                            &e.text().collect::<String>().replace('\n', " "),
                                            " ",
                                        )
                                        .trim()
                                        .to_string()
                                })
                                .unwrap_or_default();

                            for meal in &meals {
                                if meal.name == name
                                    && meal.date == date
                                    && meal.course == course
                                    && meal.description == description
                                {
                                    continue 'outer;
                                }
                            }
                            if name.is_empty() && course.is_empty() && description.is_empty() {
                                continue;
                            }

                            meals.push(Meal {
                                name,
                                date: today_string.clone(),
                                course,
                                description,
                            });
                            // println!("Meal: {name}");
                            // println!("Course: {course}");
                            // println!("Description: {description}");
                            // println!("---");
                        }
                    }
                }
            }
        }

        meals
    }

    async fn fetch_meals(&self) -> Result<String, MealClientError> {
        if self.canteen_url.is_empty() {
            return Err(MealClientError::InvalidConfig(
                "self.canteen_url is not set!".to_string(),
            ));
        }

        debug!("icanteen: sending request to: {}", self.canteen_url);

        let request = self.get_client().get(&self.canteen_url).send().await?;

        let resp = request.text().await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_parse_meals_response_with_valid_html() {
        let mut client = ICanteenClient::new("https://example.com".to_string());
        let test_date = Local.with_ymd_and_hms(2024, 11, 29, 12, 0, 0).unwrap();
        client.date = Some(test_date);

        let html = r#"
            <div class="jidelnicekDen">
                <div class="jidelnicekTop semibold" id="day-2024-11-29">29. 11. 2024</div>
                <div class="container">
                    <div class="smallBoldTitle"><span>Polévka</span></div>
                    <div class="shrinkedColumn jidelnicekItem"><span style="color: green">1</span></div>
                    <div class="column jidelnicekItem">Hovězí vývar s nudlemi</div>
                </div>
            </div>
        "#;

        let meals = client.parse_meals_response(html.to_string());
        assert_eq!(meals.len(), 1);
        assert_eq!(meals[0].name, "Polévka");
        assert_eq!(meals[0].course, "1");
        assert_eq!(meals[0].description, "Hovězí vývar s nudlemi");
    }

    #[test]
    fn test_parse_meals_response_empty_html() {
        let mut client = ICanteenClient::new("https://example.com".to_string());
        let test_date = Local.with_ymd_and_hms(2024, 11, 29, 12, 0, 0).unwrap();
        client.date = Some(test_date);

        let html = "<html><body></body></html>";
        let meals = client.parse_meals_response(html.to_string());
        assert_eq!(meals.len(), 0);
    }

    #[test]
    fn test_parse_meals_response_wrong_date() {
        let mut client = ICanteenClient::new("https://example.com".to_string());
        let test_date = Local.with_ymd_and_hms(2024, 11, 29, 12, 0, 0).unwrap();
        client.date = Some(test_date);

        let html = r#"
            <div class="jidelnicekDen">
                <div class="jidelnicekTop semibold" id="day-2024-11-28">28. 11. 2024</div>
                <div class="container">
                    <div class="smallBoldTitle"><span>Polévka</span></div>
                </div>
            </div>
        "#;

        let meals = client.parse_meals_response(html.to_string());
        assert_eq!(meals.len(), 0);
    }

    #[test]
    fn test_parse_meals_response_duplicate_filtering() {
        let mut client = ICanteenClient::new("https://example.com".to_string());
        let test_date = Local.with_ymd_and_hms(2024, 11, 29, 12, 0, 0).unwrap();
        client.date = Some(test_date);

        let html = r#"
            <div class="jidelnicekDen">
                <div class="jidelnicekTop semibold" id="day-2024-11-29">29. 11. 2024</div>
                <div class="container">
                    <div class="smallBoldTitle"><span>Polévka</span></div>
                    <div class="shrinkedColumn jidelnicekItem"><span style="color: green">1</span></div>
                    <div class="column jidelnicekItem">Hovězí vývar</div>
                </div>
                <div class="container">
                    <div class="smallBoldTitle"><span>Pizza</span></div>
                    <div class="shrinkedColumn jidelnicekItem"><span style="color: green">2</span></div>
                    <div class="column jidelnicekItem">Sýrová pizza</div>
                </div>
            </div>
        "#;

        let meals = client.parse_meals_response(html.to_string());
        assert_eq!(meals.len(), 2);
        assert_eq!(meals[0].name, "Polévka");
        assert_eq!(meals[1].name, "Pizza");
    }

    #[tokio::test]
    async fn test_fetch_meals_invalid_config() {
        let client = ICanteenClient::new("".to_string());
        let result = client.fetch_meals().await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MealClientError::InvalidConfig(_)
        ));
    }
}
