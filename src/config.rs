use crate::FileFinder;
use crate::errors::{Result, UserError};
use crate::gherkin::Sorter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

/// the filename of the configuration file
pub const CONFIG_FILE_NAME: &str = "cucumber-sort.json";

/// template for new config files
const TEMPLATE: &str = r#"{
  "include": [],
  "exclude": [],
  "record": false,
  "fail-fast": false,
  "steps": [],
  "unknown-steps": []
}
"#;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JsonConfig {
  #[serde(default)]
  pub include: Vec<String>,
  #[serde(default)]
  pub exclude: Vec<String>,
  #[serde(default)]
  pub record: bool,
  #[serde(default, rename = "fail-fast")]
  pub fail_fast: bool,
  #[serde(default)]
  pub steps: Vec<StepPattern>,
  #[serde(default, rename = "unknown-steps")]
  pub unknown_steps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StepPattern {
  Single(String),
  Group(Vec<String>),
}

pub struct Config {
  pub finder: FileFinder,
  pub sorter: Sorter,
  pub record: bool,
  pub fail_fast: bool,
}

pub fn load() -> Result<Config> {
  let json_config = load_json_config()?;
  Ok(Config {
    finder: FileFinder::from_json_config(&json_config)?,
    sorter: Sorter::from_json_config(&json_config)?,
    record: json_config.record,
    fail_fast: json_config.fail_fast,
  })
}

pub fn load_json_config() -> Result<JsonConfig> {
  match fs::read_to_string(CONFIG_FILE_NAME) {
    Ok(text) => serde_json::from_str(&text).map_err(|err| UserError::ConfigFileRead {
      file: CONFIG_FILE_NAME.into(),
      reason: format!("Invalid JSON: {}", err),
    }),
    Err(err) => match err.kind() {
      ErrorKind::NotFound => Ok(JsonConfig::default()),
      _ => Err(UserError::ConfigFileRead {
        file: CONFIG_FILE_NAME.into(),
        reason: err.to_string(),
      }),
    },
  }
}

pub fn save_json_config(config: &JsonConfig) -> Result<()> {
  let json = serde_json::to_string_pretty(config).map_err(|err| UserError::ConfigFileCreate {
    file: CONFIG_FILE_NAME.into(),
    message: format!("Failed to serialize config: {}", err),
  })?;
  fs::write(CONFIG_FILE_NAME, json).map_err(|err| UserError::ConfigFileCreate {
    file: CONFIG_FILE_NAME.into(),
    message: err.to_string(),
  })
}

pub fn create() -> Result<()> {
  fs::write(CONFIG_FILE_NAME, TEMPLATE).map_err(|err| UserError::ConfigFileCreate {
    file: CONFIG_FILE_NAME.into(),
    message: err.to_string(),
  })
}
