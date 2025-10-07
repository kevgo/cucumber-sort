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
    Ok(text) => {
      let sanitized = strip_comments(&text);
      serde_json::from_str(&sanitized).map_err(|err| UserError::ConfigFileRead {
        file: CONFIG_FILE_NAME.into(),
        reason: format!("Invalid JSON: {}", err),
      })
    }
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

/// Strips single-line (//) and multi-line (/* */) comments from JSON text,
/// replacing them with spaces to preserve line numbers for error reporting.
fn strip_comments(text: &str) -> String {
  let mut result = String::with_capacity(text.len());
  let mut chars = text.chars().peekable();

  while let Some(ch) = chars.next() {
    if ch == '/' {
      match chars.peek() {
        Some(&'/') => {
          // Single-line comment: replace with spaces until newline
          result.push(' ');
          chars.next(); // consume second '/'
          result.push(' ');
          while let Some(&next_ch) = chars.peek() {
            if next_ch == '\n' {
              result.push(chars.next().unwrap());
              break;
            }
            chars.next();
            result.push(' ');
          }
        }
        Some(&'*') => {
          // Multi-line comment: replace with spaces, preserve newlines
          result.push(' ');
          chars.next(); // consume '*'
          result.push(' ');
          let mut prev_was_star = false;
          while let Some(next_ch) = chars.next() {
            if prev_was_star && next_ch == '/' {
              result.push(' ');
              break;
            }
            prev_was_star = next_ch == '*';
            result.push(if next_ch == '\n' { '\n' } else { ' ' });
          }
        }
        _ => result.push(ch),
      }
    } else if ch == '"' {
      // Inside a string: copy everything as-is until closing quote
      result.push(ch);
      let mut escaped = false;
      while let Some(next_ch) = chars.next() {
        result.push(next_ch);
        if escaped {
          escaped = false;
        } else if next_ch == '\\' {
          escaped = true;
        } else if next_ch == '"' {
          break;
        }
      }
    } else {
      result.push(ch);
    }
  }

  result
}
