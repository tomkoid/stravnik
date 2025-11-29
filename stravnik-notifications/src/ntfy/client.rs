pub struct NtfyClient {
    pub host_url: String,
    pub room: String,
}

impl NtfyClient {
    pub fn new(host_url: String, room: String) -> NtfyClient {
        NtfyClient { host_url, room }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntfy_client_new() {
        let client = NtfyClient::new(
            "https://ntfy.sh".to_string(),
            "test-room".to_string(),
        );
        assert_eq!(client.host_url, "https://ntfy.sh");
        assert_eq!(client.room, "test-room");
    }

    #[test]
    fn test_ntfy_client_empty_values() {
        let client = NtfyClient::new("".to_string(), "".to_string());
        assert_eq!(client.host_url, "");
        assert_eq!(client.room, "");
    }

    #[test]
    fn test_ntfy_client_with_trailing_slash() {
        let client = NtfyClient::new(
            "https://ntfy.sh/".to_string(),
            "room".to_string(),
        );
        assert_eq!(client.host_url, "https://ntfy.sh/");
    }
}
