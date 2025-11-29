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

#[cfg(test)]
mod tests {
    use super::*;
    use stravnik_core::meal_data::Meal;
    use stravnik_core::services::MealListService;

    fn create_test_meals_list() -> MealsList {
        let meals = vec![
            Meal {
                name: "".to_string(),
                date: "29.11.2024".to_string(),
                course: "Polévka".to_string(),
                description: "Hovězí vývar s nudlemi".to_string(),
            },
            Meal {
                name: "Pizza".to_string(),
                date: "29.11.2024".to_string(),
                course: "Hlavní chod".to_string(),
                description: "Italská pizza s mozzarellou".to_string(),
            },
        ];
        MealsList::new(meals, MealListService::Strava)
    }

    #[test]
    fn test_basic_fmt() {
        let meals_list = create_test_meals_list();
        let formatted = meals_list.basic_fmt();

        assert!(formatted.contains("### Obědy pro **29.11.2024**:"));
        assert!(formatted.contains("0. *[Polévka]* Hovězí vývar s nudlemi"));
        assert!(formatted.contains("1. *[Hlavní chod]* Italská pizza s mozzarellou"));
    }

    #[test]
    fn test_discord_fmt_structure() {
        let meals_list = create_test_meals_list();
        let payload = meals_list.discord_fmt();

        assert!(payload.is_object());
        assert!(payload.get("embeds").is_some());
        assert!(payload["embeds"].is_array());
        assert_eq!(payload["embeds"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_discord_fmt_embed_content() {
        let meals_list = create_test_meals_list();
        let payload = meals_list.discord_fmt();

        let embed = &payload["embeds"][0];
        assert_eq!(embed["title"], "Obědy pro **29.11.2024**");
        assert_eq!(embed["color"], 0xcba6f7);

        let description = embed["description"].as_str().unwrap();
        assert!(description.contains("*[Polévka]* Hovězí vývar s nudlemi"));
        assert!(description.contains("*[Hlavní chod - **Pizza**]* Italská pizza s mozzarellou"));
    }

    #[test]
    fn test_discord_fmt_no_meal_name() {
        let meals = vec![Meal {
            name: "".to_string(),
            date: "29.11.2024".to_string(),
            course: "Polévka".to_string(),
            description: "Soup".to_string(),
        }];
        let meals_list = MealsList::new(meals, MealListService::Strava);
        let payload = meals_list.discord_fmt();

        let description = payload["embeds"][0]["description"].as_str().unwrap();
        assert!(description.contains("*[Polévka]* Soup"));
        assert!(!description.contains("**"));
    }

    #[test]
    fn test_basic_fmt_single_meal() {
        let meals = vec![Meal {
            name: "".to_string(),
            date: "01.01.2024".to_string(),
            course: "Dessert".to_string(),
            description: "Ice cream".to_string(),
        }];
        let meals_list = MealsList::new(meals, MealListService::ICanteen);
        let formatted = meals_list.basic_fmt();

        assert!(formatted.contains("### Obědy pro **01.01.2024**:"));
        assert!(formatted.contains("0. *[Dessert]* Ice cream"));
    }
}
