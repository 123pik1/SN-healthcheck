use serde::Deserialize;
use std::{fmt::Display, fs::File};

#[derive(Debug, Deserialize)]
pub struct Check {
    name: String,
    description: String,
    json_path: String,
    // In checks there can ratio be checked
    // F.e. what percent of workers are working on the same task
    warning_threshold: f64,
    critical_threshold: f64,
    enabled: bool,
}

/// instance_url, endpoint and api_key are the names of environmental variables
#[derive(Debug, Deserialize)]
pub struct InstanceConfig {
    pub instance_url: String,
    pub endpoint: String,
    pub api_key: String,

    checks: Vec<Check>,
}

#[derive(Debug, Deserialize)]
struct Defaults {
    api_key: String,
    instance_url: String,
    endpoint: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    defaults: Option<Defaults>,
    pub instances: Vec<InstanceConfig>,
}

impl ConfigFile {
    pub fn load_from_yaml(filepath: &str) -> ConfigFile {
        let file = File::open(filepath).expect("No file found");
        let config: ConfigFile =
            serde_yaml::from_reader(file).expect("There was an error reading a file");
        config
    }
}
