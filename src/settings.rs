use std::env;

pub struct Settings {
    pub config_path: String,
    pub timeout: u64,
    pub timeout_increase: u64,
    pub do_increase_timeout_after_fail: bool,
    pub timeout_max_increase_times: u32,
    pub sn_api_key: String,
    pub sn_instance_url: String,
    pub sn_api_url: String,
}

impl Settings {
    /// Environemntal variables must be loaded when function is called
    pub fn load_from_env() -> Self {
        Settings {
            config_path: env::var("CONFIG_PATH").unwrap_or("./config/config.yaml".to_string()),
            timeout: env::var("TIMEOUT")
                .ok()
                .and_then(|str| str.parse().ok())
                .unwrap_or(30),
            timeout_increase: env::var("TIMEOUT_INCREASE")
                .ok()
                .and_then(|str| str.parse().ok())
                .unwrap_or(15),
            do_increase_timeout_after_fail: env::var("DO_INCREASE_TIMEOUT_AFTER_FAIL")
                .ok()
                .and_then(|str| str.parse().ok())
                .unwrap_or(true),
            timeout_max_increase_times: env::var("TIMEOUT_MAX_INCREASE_TIMES")
                .ok()
                .and_then(|str| str.parse().ok())
                .unwrap_or(5),
            sn_api_key: env::var("SN_API_KEY").unwrap_or("".to_string()),
            sn_instance_url: env::var("SN_INSTANCE_URL").unwrap_or("".to_string()),
            sn_api_url: env::var("SN_API_URL").unwrap_or("".to_string()),
        }
    }
}
