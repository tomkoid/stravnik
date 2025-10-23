pub struct NtfyClient {
    pub host_url: String,
    pub room: String,
}

impl NtfyClient {
    pub fn new(host_url: String, room: String) -> NtfyClient {
        NtfyClient { host_url, room }
    }
}
