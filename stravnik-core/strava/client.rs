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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strava_client_new() {
        let client = StravaClient::new("12345".to_string());
        assert_eq!(client.canteen_id, "12345");
        assert_eq!(client.s5url, None);
    }

    #[test]
    fn test_strava_client_get_client() {
        let client = StravaClient::new("12345".to_string());
        let _http_client = client.get_client();
    }

    #[test]
    fn test_strava_client_empty_canteen_id() {
        let client = StravaClient::new("".to_string());
        assert_eq!(client.canteen_id, "");
    }
}
