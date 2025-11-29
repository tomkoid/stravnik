use chrono::Datelike;

pub trait ToDateStringExt {
    fn to_date_string(&self) -> String;
}

impl<T: chrono::TimeZone> ToDateStringExt for chrono::DateTime<T> {
    fn to_date_string(&self) -> String {
        format!("{:02}.{:02}.{}", self.day(), self.month(), self.year())
    }
}
