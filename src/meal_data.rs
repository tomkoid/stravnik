use serde_json::json;

use crate::services::MealListService;

pub struct MealsList {
    pub meals: Vec<Meal>,
    pub source: MealListService,
}

impl MealsList {
    pub fn new(meals: Vec<Meal>, source: MealListService) -> Self {
        Self { meals, source }
    }

    pub fn basic_fmt(&self) -> String {
        let mut text = format!("### Obědy pro **{}**:\n", self.meals[0].date);

        for (index, meal) in self.meals.iter().enumerate() {
            text = format!(
                "{}{}. *[{}]* {}\n",
                text, index, meal.course, meal.description
            );
        }

        text
    }

    pub fn discord_fmt(&self) -> serde_json::Value {
        let mut text = String::new();
        for (index, meal) in self.meals.iter().enumerate() {
            let meal_name = if meal.name.is_empty() {
                "".to_string()
            } else {
                format!(" - **{}**", meal.name)
            };

            text = format!(
                "{}{}. *[{}{}]* {}\n",
                text, index, meal.course, meal_name, meal.description
            );
        }

        let title = format!("Obědy pro **{}**", self.meals[0].date);

        let payload = json!({
            "embeds": [{
                "title": title,
                "description": text,
                "color": 0xcba6f7
            }]
        });

        payload
    }
}

pub struct Meal {
    pub name: String,
    pub date: String,
    pub course: String,
    pub description: String,
}
