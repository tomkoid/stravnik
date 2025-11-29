use serde_json::json;
use stravnik_core::meal_data::MealsList;

pub trait MealsListFormatter {
    fn basic_fmt(&self) -> String;
    fn discord_fmt(&self) -> serde_json::Value;
}

impl MealsListFormatter for MealsList {
    fn basic_fmt(&self) -> String {
        let mut text = format!("### Obědy pro **{}**:\n", self.meals[0].date);

        for (index, meal) in self.meals.iter().enumerate() {
            text = format!(
                "{}{}. *[{}]* {}\n",
                text, index, meal.course, meal.description
            );
        }

        text
    }

    fn discord_fmt(&self) -> serde_json::Value {
        let mut text = String::new();
        for meal in self.meals.iter() {
            let meal_name = if meal.name.is_empty() {
                "".to_string()
            } else {
                format!(" - **{}**", meal.name)
            };

            text = format!(
                "{}*[{}{}]* {}\n",
                text, meal.course, meal_name, meal.description
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
