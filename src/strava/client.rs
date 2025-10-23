pub struct StravaClient {
    pub canteen_id: String,
    pub s5url: Option<String>,

    client: reqwest::Client,
}

impl StravaClient {
    pub fn new(canteen_id: String) -> Self {
        let client = reqwest::Client::new();

        Self {
            canteen_id,
            s5url: None,
            client,
        }
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }
}
