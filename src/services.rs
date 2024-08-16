use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Service {
    Matrix,
    Ntfy,
}

pub fn get_notification_services() -> Vec<Service> {
    let default: &str = "false";

    let mut services: Vec<Service> = Vec::new();

    for service in Service::iter() {
        if std::env::var(format!("{:?}_ENABLE", service).to_uppercase())
            .unwrap_or(default.to_owned())
            == "true"
        {
            services.push(service);
        }
    }

    services
}
