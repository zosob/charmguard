use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RasaRequest<'a> {
    sender: &'a str,
    message: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct RasaResponse {
    pub text: String,
}

pub async fn send_message_to_rasa(sender: &str, message: &str) -> Result<Vec<RasaResponse>, reqwest::Error> {
    let client = Client::new();
    let body = RasaRequest { sender, message };

    let resp = client
        .post("http://localhost:5005/webhooks/rest/webhook")
        .json(&body)
        .send()
        .await?;

    resp.json::<Vec<RasaResponse>>().await
}
