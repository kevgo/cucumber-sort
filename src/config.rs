use crate::FileFinder;
use crate::cli::Flags;
use crate::errors::{Result, UserError};
use crate::gherkin::Sorter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

/// the filename of the configuration file
pub const CONFIG_FILE_NAME: &str = "cucumber-sort.json";

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

impl Config {
  /// merges the given CLI flags into the configuration
  pub fn merge(&mut self, flags: Flags) {
    if flags.fail_fast {
      self.fail_fast = true;
    }
    if flags.record {
      self.record = true;
    }
  }
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
        content: text,
        reason: format!("Invalid JSON: {}", err),
      })
    }
    Err(err) => match err.kind() {
      ErrorKind::NotFound => Ok(JsonConfig::default()),
      _ => Err(UserError::ConfigFileRead {
        file: CONFIG_FILE_NAME.into(),
        content: String::new(),
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
  save_json_config(&JsonConfig::default())
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
        _ => result.push(ch),
      }
    } else if ch == '"' {
      // Inside a string: copy everything as-is until closing quote
      result.push(ch);
      let mut escaped = false;
      for next_ch in chars.by_ref() {
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

#[cfg(test)]
mod tests {
  use super::*;

  mod strip_comments {
    #[test]
    fn no_comments() {
      let give = r#"{"key": "value"}"#;
      assert_eq!(super::strip_comments(give), give);
    }

    #[test]
    fn single_line_comment_at_end() {
      let give = r#"{"key": "value"} // this is a comment"#;
      let want = r#"{"key": "value"}                     "#;
      assert_eq!(super::strip_comments(give), want);
    }

    #[test]
    fn comment_on_own_line() {
      let give = "// comment\n{\"key\": \"value\"}";
      let want = "          \n{\"key\": \"value\"}";
      assert_eq!(super::strip_comments(give), want);
    }

    #[test]
    fn multiple_comments() {
      let give = r#"{
  // first comment
  "key": "value", // inline comment
  // another comment
  "key2": "value2"
}"#;
      let want = "{\n                  \n  \"key\": \"value\",                  \n                    \n  \"key2\": \"value2\"\n}";
      assert_eq!(super::strip_comments(give), want);
    }

    #[test]
    fn preserves_slashes_in_strings() {
      let give = r#"{"url": "https://example.com", "comment": "// not a comment"}"#;
      assert_eq!(super::strip_comments(give), give);
    }

    #[test]
    fn escaped_quotes_in_strings() {
      let give = r#"{"text": "She said \"hello\" // still in string"}"#;
      assert_eq!(super::strip_comments(give), give);
    }

    #[test]
    fn comment_at_eof_no_newline() {
      let give = r#"{"key": "value"} // comment"#;
      let want = r#"{"key": "value"}           "#;
      assert_eq!(super::strip_comments(give), want);
    }

    #[test]
    fn empty_string() {
      assert_eq!(super::strip_comments(""), "");
    }

    #[test]
    fn only_comment() {
      let give = "// just a comment";
      let want = "                 ";
      assert_eq!(super::strip_comments(give), want);
    }

    #[test]
    fn single_slash() {
      let give = r#"{"path": "/home/user"}"#;
      assert_eq!(super::strip_comments(give), give);
    }

    #[test]
    fn backslash_before_quote() {
      let give = r#"{"pattern": "\\\"}"#;
      assert_eq!(super::strip_comments(give), give);
    }

    #[test]
    fn complex_json() {
      let give = r#"{
  // Configuration file
  "include": ["*.feature"], // glob patterns
  "exclude": [], // none
  "record": false // don't record
}"#;
      let want = "{\n                       \n  \"include\": [\"*.feature\"],                 \n  \"exclude\": [],        \n  \"record\": false                \n}";
      assert_eq!(super::strip_comments(give), want);
    }
  }
}
