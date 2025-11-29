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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icanteen_client_new() {
        let client = ICanteenClient::new("https://example.com".to_string());
        assert_eq!(client.canteen_url, "https://example.com");
        assert_eq!(client.date, None);
    }

    #[test]
    fn test_icanteen_client_get_client() {
        let client = ICanteenClient::new("https://example.com".to_string());
        let _http_client = client.get_client();
    }

    #[test]
    fn test_icanteen_client_empty_url() {
        let client = ICanteenClient::new("".to_string());
        assert_eq!(client.canteen_url, "");
    }
}
