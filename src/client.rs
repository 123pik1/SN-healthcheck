use std::time::Duration;

use reqwest::{Url, header};
use serde_json::Value;

use crate::{client, settings::Settings};

pub struct RequestAnswer {
    pub times_requested: u32,
    pub answer: serde_json::Value,
}

pub struct Client {
    api_key: String,
    instance_url: String,
    api_url: String,
    timeout: u64,
    timeout_increase: u64,
    do_increase_timeout_after_fail: bool,
    timeout_max_increase_times: u32,
}

impl Client {
    pub fn init(api_key: &str, instance_url: &str, api_url: &str, settings: Settings) -> Self {
        Self {
            api_key: api_key.to_string(),
            instance_url: instance_url.to_string(),
            api_url: api_url.to_string(),
            timeout: settings.timeout,
            timeout_increase: settings.timeout_increase,
            do_increase_timeout_after_fail: settings.do_increase_timeout_after_fail,
            timeout_max_increase_times: settings.timeout_max_increase_times,
        }
    }

    pub fn get_stats(&self) -> RequestAnswer {
        let mut times_counter: u32 = 0;

        RequestAnswer {
            times_requested: times_counter,
            answer: self.request(5).unwrap(),
        }
    }

    fn request(&self, timeout: u64) -> Option<serde_json::Value> {
        let url = format!("{}/{}", self.instance_url, self.api_url);

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .ok()?;

        println!("Request sending with timeout {}", timeout);

        let response = client
            .get(url)
            .header("X-API-key", &self.api_key)
            .header(header::ACCEPT, "application/json")
            .send()
            .ok()?
            .json::<serde_json::Value>()
            .ok()?;
        Some(response)
    }
}
