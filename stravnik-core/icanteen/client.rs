pub struct ICanteenClient {
    pub canteen_url: String,

    pub date: Option<chrono::DateTime<chrono::Local>>,
    client: reqwest::Client,
}

impl ICanteenClient {
    pub fn new(canteen_url: String) -> Self {
        let client = reqwest::Client::new();

        Self {
            canteen_url,
            date: None,
            client,
        }
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }
}
