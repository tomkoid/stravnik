use chrono::Datelike;

pub trait ToDateStringExt {
    fn to_date_string(&self) -> String;
}

impl<T: chrono::TimeZone> ToDateStringExt for chrono::DateTime<T> {
    fn to_date_string(&self) -> String {
        format!("{:02}.{:02}.{}", self.day(), self.month(), self.year())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, TimeZone};

    #[test]
    fn test_to_date_string_formatting() {
        let date = Local.with_ymd_and_hms(2024, 3, 5, 12, 0, 0).unwrap();
        assert_eq!(date.to_date_string(), "05.03.2024");
    }

    #[test]
    fn test_to_date_string_double_digits() {
        let date = Local.with_ymd_and_hms(2024, 11, 29, 12, 0, 0).unwrap();
        assert_eq!(date.to_date_string(), "29.11.2024");
    }

    #[test]
    fn test_to_date_string_single_digit_month() {
        let date = Local.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        assert_eq!(date.to_date_string(), "15.01.2024");
    }
}
