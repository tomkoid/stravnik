use crate::{
    icanteen::client::ICanteenClient,
    meal_data::{Meal, MealsList},
    utils::today_string,
};
use anyhow::anyhow;
use chrono::Local;
use log::{debug, info};
use regex::Regex;
use scraper::{Html, Selector};
use stravnik_core::services::MealListService;

impl ICanteenClient {
    pub async fn get_meals(&self) -> anyhow::Result<MealsList> {
        info!("icanteen: getting meals...");

        let resp = self.fetch_meals().await?;
        let meals = self.parse_meals_response(resp);

        info!("icanteen: got meals!");

        Ok(MealsList::new(meals, MealListService::ICanteen))
    }

    fn parse_meals_response(&self, meals_resp: String) -> Vec<Meal> {
        let mut meals: Vec<Meal> = Vec::new();

        let document = Html::parse_document(&meals_resp);

        let today = Local::now().date_naive();
        let today_string = today_string();

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

    async fn fetch_meals(&self) -> anyhow::Result<String> {
        if self.canteen_url.is_empty() {
            return Err(anyhow!("self.canteen_url is not set!"));
        }

        debug!("icanteen: sending request to: {}", self.canteen_url);

        let request = self.get_client().get(&self.canteen_url).send().await?;

        let resp = request.text().await?;

        Ok(resp)
    }
}
