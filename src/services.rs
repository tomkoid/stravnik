pub enum Service {
    Matrix,
    Ntfy,
}

pub fn get_notification_services() -> Vec<Service> {
    vec![Service::Matrix, Service::Ntfy]
}
