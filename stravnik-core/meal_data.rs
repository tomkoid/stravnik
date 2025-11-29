use crate::services::MealListService;

pub struct MealsList {
    pub meals: Vec<Meal>,
    pub source: MealListService,
}

impl MealsList {
    pub fn new(meals: Vec<Meal>, source: MealListService) -> Self {
        Self { meals, source }
    }
}

#[derive(Clone)]
pub struct Meal {
    pub name: String,
    pub date: String,
    pub course: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_meal() -> Meal {
        Meal {
            name: "Pizza".to_string(),
            date: "29.11.2024".to_string(),
            course: "Hlavní chod".to_string(),
            description: "Italská pizza s mozzarellou".to_string(),
        }
    }

    #[test]
    fn test_meals_list_new() {
        let meals = vec![create_test_meal()];
        let meals_list = MealsList::new(meals.clone(), MealListService::Strava);

        assert_eq!(meals_list.meals.len(), 1);
        assert_eq!(meals_list.meals[0].name, "Pizza");
        assert!(matches!(meals_list.source, MealListService::Strava));
    }

    #[test]
    fn test_meals_list_with_icanteen() {
        let meals = vec![create_test_meal()];
        let meals_list = MealsList::new(meals, MealListService::ICanteen);

        assert!(matches!(meals_list.source, MealListService::ICanteen));
    }

    #[test]
    fn test_meals_list_empty() {
        let meals_list = MealsList::new(vec![], MealListService::Strava);
        assert_eq!(meals_list.meals.len(), 0);
    }

    #[test]
    fn test_meals_list_multiple_meals() {
        let meals = vec![
            Meal {
                name: "Soup".to_string(),
                date: "29.11.2024".to_string(),
                course: "Polévka".to_string(),
                description: "Chicken soup".to_string(),
            },
            create_test_meal(),
        ];
        let meals_list = MealsList::new(meals, MealListService::Strava);

        assert_eq!(meals_list.meals.len(), 2);
        assert_eq!(meals_list.meals[0].course, "Polévka");
        assert_eq!(meals_list.meals[1].course, "Hlavní chod");
    }
}
