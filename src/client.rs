use std::time::Duration;

use reqwest::header;

use crate::settings::Settings;

pub struct RequestAnswer {
    pub times_requested: u32,
    pub answer: Option<serde_json::Value>,
}

pub struct Client {
    api_key: String,
    instance_url: String,
    api_url: String,
    timeout: u64,
    timeout_increase: u64,
    do_increase_timeout_after_fail: bool,
    timeout_max_increase_times: u32,
    api_key_header_name: String,
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
            api_key_header_name: settings.sn_api_key_header,
        }
    }

    pub fn get_stats(&self) -> RequestAnswer {
        let mut current_timeout = self.timeout;

        for times in 0..self.timeout_max_increase_times {
            if let Some(value) = self.request(current_timeout) {
                return RequestAnswer {
                    times_requested: times + 1,
                    answer: Some(value),
                };
            }
            if self.do_increase_timeout_after_fail {
                current_timeout += self.timeout_increase;
            }
        }

        RequestAnswer {
            times_requested: self.timeout_max_increase_times,
            answer: None,
        }
    }

    fn request(&self, timeout: u64) -> Option<serde_json::Value> {
        let instance_url = if self.instance_url.starts_with("http://")
            || self.instance_url.starts_with("https://")
        {
            self.instance_url.trim_end_matches('/').to_string()
        } else {
            format!("https://{}", self.instance_url.trim_end_matches('/'))
        };
        let url = format!("{}/{}", instance_url, self.api_url.trim_start_matches('/'));

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|error| eprintln!("Could not build HTTP client: {error}"))
            .ok()?;

        println!("Sending request to {url} with timeout {timeout}s");

        let response = client
            .get(url)
            .header(&self.api_key_header_name, &self.api_key)
            .header(header::ACCEPT, "application/json")
            .send()
            .map_err(|error| eprintln!("Request failed before receiving a response: {error}"))
            .ok()?;

        let status = response.status();
        let body = response
            .text()
            .map_err(|error| eprintln!("Could not read response body: {error}"))
            .ok()?;

        if !status.is_success() {
            eprintln!("ServiceNow returned {status}: {body}");
            return None;
        }

        let response = serde_json::from_str(&body)
            .map_err(|error| eprintln!("Invalid JSON response: {error}; body: {body}"))
            .ok()?;

        Some(response)
    }
}
