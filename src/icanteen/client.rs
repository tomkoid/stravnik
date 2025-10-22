pub struct ICanteenClient {
    pub canteen_url: String,

    client: reqwest::Client,
}

impl ICanteenClient {
    pub fn new() -> Self {
        let canteen_url = std::env::var("ICANTEEN_URL").unwrap();
        let client = reqwest::Client::new();

        Self {
            canteen_url,
            client,
        }
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }
}
