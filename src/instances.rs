use jsonptr::Pointer;
use serde::Deserialize;
use std::{fmt::Display, fs::File};

use crate::instances::{
    CheckedState::NotEvaluated,
    Operator::{Higher, HigherOrEqual},
};

pub struct CheckResult {
    pub check_name: String,
    pub description: String,
    pub state: CheckedState,
}

pub enum CheckedState {
    Ok,
    Warning,
    Critical,
    NotEvaluated,
}

impl Display for CheckedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Warning => write!(f, "WARNING"),
            Self::Critical => write!(f, "CRITICAL"),
            Self::NotEvaluated => write!(f, "NOT EVALUATED"),
        }
    }
}

enum Operator {
    Lower,
    LowerOrEqual,
    Equal,
    HigherOrEqual,
    Higher,
}

impl Operator {
    /// Defailts to HigherOrEqual
    pub fn from_str(operator: &str) -> Self {
        match operator.to_uppercase().as_str() {
            "LOWER" => Self::Lower,
            "LOWER_OR_EQUAL" => Self::LowerOrEqual,
            "EQUAL" => Self::Equal,
            "HIGHER_OR_EQUAL" => Self::HigherOrEqual,
            "HIGHER" => Higher,
            _ => HigherOrEqual,
        }
    }

    /// If nmb1 matches condition
    pub fn compare(&self, nmb1: f64, nmb2: f64) -> bool {
        match self {
            Self::Lower => nmb1 < nmb2,
            Self::LowerOrEqual => nmb1 <= nmb2,
            Self::Equal => nmb1 == nmb2,
            Self::HigherOrEqual => nmb1 >= nmb2,
            Self::Higher => nmb1 > nmb2,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Check {
    name: String,
    description: String,
    json_path: String,
    // In checks there can ratio be checked
    // F.e. what percent of workers are working on the same task
    warning_threshold: f64,
    critical_threshold: f64,
    operator: String,
    enabled: bool,
}

impl Check {
    pub fn do_check(&self, json_response: &serde_json::Value) -> CheckResult {
        if !self.enabled {
            return CheckResult {
                check_name: self.name.clone(),
                description: self.description.clone(),
                state: NotEvaluated,
            };
        }

        let op = Operator::from_str(&self.operator);

        if let Ok(ptr) = Pointer::parse(&self.json_path)
            && let Ok(val) = ptr.resolve(json_response)
            && let Some(got_value) = val.as_f64()
        {
            if op.compare(got_value, self.warning_threshold) {
                return CheckResult {
                    check_name: self.name.clone(),
                    description: self.description.clone(),
                    state: CheckedState::Warning,
                };
            }
            if op.compare(got_value, self.critical_threshold) {
                return CheckResult {
                    check_name: self.name.clone(),
                    description: self.description.clone(),
                    state: CheckedState::Critical,
                };
            }
        } else {
            return CheckResult {
                check_name: self.name.clone(),
                description: self.description.clone(),
                state: NotEvaluated,
            };
        }

        CheckResult {
            check_name: self.name.clone(),
            description: self.description.clone(),
            state: CheckedState::Ok,
        }
    }
}

/// instance_url, endpoint and api_key are the names of environmental variables
#[derive(Debug, Deserialize)]
pub struct InstanceConfig {
    pub instance_url: String,
    pub endpoint: String,
    pub api_key: String,

    checks: Vec<Check>,
}

impl InstanceConfig {
    pub fn do_checks(&self, json_response: serde_json::Value) -> Vec<CheckResult> {
        let mut results: Vec<CheckResult> = Vec::new();

        for check in self.checks.iter() {
            results.push(check.do_check(&json_response));
        }

        results
    }
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
