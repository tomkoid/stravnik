use chrono::{Datelike, Local};

pub fn today_string() -> String {
    let now = Local::now();
    format!("{:02}.{:02}.{}", now.day(), now.month(), now.year())
}
