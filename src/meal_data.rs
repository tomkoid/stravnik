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

pub struct Meal {
    pub name: String,
    pub date: String,
    pub course: String,
}
