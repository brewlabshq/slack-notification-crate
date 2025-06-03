use std::env;

use thiserror::Error;

pub struct Config {
    pub slack_token: String,
}

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Error: Invalid Slack Token")]
    InvalidSlackToken
}

impl Config {
    pub fn get_config() -> Self {
        let slack_token = env::var("SLACK_TOKEN").unwrap_or_else(|_| ConfigError::InvalidSlackToken.to_string());

        Self { slack_token }
    }
}