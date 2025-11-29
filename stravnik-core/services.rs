use clap::ValueEnum;
use serde::Serialize;

#[derive(Default, ValueEnum, Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MealListService {
    #[default]
    Strava,

    ICanteen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meal_list_service_default() {
        let service: MealListService = Default::default();
        assert_eq!(service, MealListService::Strava);
    }

    #[test]
    fn test_meal_list_service_clone() {
        let service = MealListService::ICanteen;
        let cloned = service.clone();
        assert_eq!(service, cloned);
    }

    #[test]
    fn test_meal_list_service_serialize() {
        let service = MealListService::Strava;
        let serialized = serde_json::to_string(&service).unwrap();
        assert_eq!(serialized, "\"strava\"");

        let service = MealListService::ICanteen;
        let serialized = serde_json::to_string(&service).unwrap();
        assert_eq!(serialized, "\"i-canteen\"");
    }
}
