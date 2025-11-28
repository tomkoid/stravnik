use log::debug;
use serde::Serialize;

use crate::strava::client::StravaClient;

#[derive(Serialize, Debug)]
struct S4PolozkyRequest {
    cislo: String,
    lang: String,
    polozky: String,
}

impl StravaClient {
    pub async fn fetch_s5url(&mut self) {
        let req_payload = S4PolozkyRequest {
            cislo: self.canteen_id.clone(),
            lang: "EN".to_string(),
            polozky: "URLWSDL_S-URL".to_string(),
        };

        debug!("S4 Polozky request payload: {:?}", req_payload);

        let request = self
            .get_client()
            .post("https://app.strava.cz/api/s4Polozky")
            .header("Content-Type", "text/plain;charset=UTF-8")
            .body(serde_json::to_string(&req_payload).unwrap())
            .send()
            .await
            .unwrap();

        let json: serde_json::Value = serde_json::from_str(&request.text().await.unwrap()).unwrap();
        let s5url = json["urlwsdl_s"][0].as_str().unwrap().to_string();

        debug!("S5 URL: {}", s5url);

        self.s5url = Some(s5url);
    }
}
