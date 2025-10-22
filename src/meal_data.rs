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
            text = format!("{}{}. *[{}]* {}\n", text, index, meal.course, meal.name);
        }

        text
    }
}

pub struct Meal {
    pub name: String,
    pub date: String,
    pub course: String,
}
