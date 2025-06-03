use {
    crate::config::Config,
    anyhow::{Error},
    reqwest::{
        header::{HeaderMap, HeaderValue, CONTENT_TYPE},
        Client,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Debug)]
struct SlackMessage<'a> {
    channel: &'a str,
    text: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct SlackMessageResponse {
    text: String,
    bot_id: String,
    #[serde(rename = "type")]
    type_field: String,
    ts: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SlackApiResponse {
    ok: bool,
    channel: String,
    ts: String,
    message: SlackMessageResponse,
}

pub const SLACK_URL: &str = "https://slack.com/api/chat.postMessage";

pub async fn send_message(channel_id: &str, text: &str) -> Result<(), Error> {
    let token = Config::get_config().slack_token;

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"));

    let message = SlackMessage {
        channel: channel_id,
        text
    };

    let res = match client
        .post(SLACK_URL)
        .headers(headers)
        .bearer_auth(token)
        .json(&message)
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            log::error!(
                "Failed to send message to slack. Failed with error: {}",
                err
            );
            return Err(Error::msg("Failed to send message to slack"));
        }
    };

    let data: SlackApiResponse = match res.json().await {
          Ok(response) => response,
          Err(err) => {
            log::error!("Failedd to parse slack api response. Failed with error: {}", err);
            return Err(Error::msg("Failed to parse slack api response"))
          }
    };

    log::info!("Sent message to slack successfully. Response: {:#?}", data);

    Ok(())
}