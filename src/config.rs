use crate::cli::Flags;
use crate::errors::{Result, UserError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

/// the filename of the configuration file
pub const CONFIG_FILE_NAME: &str = "cucumber-sort.json";

pub fn create() -> Result<()> {
  Config::default().save()
}

/// low-level configuration, structured as in the config file
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
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

impl Config {
  pub fn load() -> Result<Config> {
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
        ErrorKind::NotFound => Ok(Config::default()),
        _ => Err(UserError::ConfigFileRead {
          file: CONFIG_FILE_NAME.into(),
          content: String::new(),
          reason: err.to_string(),
        }),
      },
    }
  }

  /// merges the given CLI flags into the configuration
  pub fn merge(self, Flags { fail_fast, record }: Flags) -> Self {
    Config {
      include: self.include,
      exclude: self.exclude,
      record: self.record || record,
      fail_fast: self.fail_fast || fail_fast,
      steps: self.steps,
      unknown_steps: self.unknown_steps,
    }
  }

  pub fn save(&self) -> Result<()> {
    let json = serde_json::to_string_pretty(self).map_err(|err| UserError::ConfigFileCreate {
      file: CONFIG_FILE_NAME.into(),
      message: format!("Failed to serialize config: {}", err),
    })?;
    fs::write(CONFIG_FILE_NAME, json).map_err(|err| UserError::ConfigFileCreate {
      file: CONFIG_FILE_NAME.into(),
      message: err.to_string(),
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StepPattern {
  Single(String),
  Group(Vec<String>),
}

/// Strips single-line comments from JSON text,
/// replacing them with spaces to preserve line numbers for error reporting.
fn strip_comments(text: &str) -> String {
  let mut result = String::with_capacity(text.len());
  let mut chars = text.chars();
  while let Some(ch) = chars.next() {
    match ch {
      '/' => {
        match chars.next() {
          Some('/') => {
            // Single-line comment: replace with spaces until newline
            result.push_str("  "); // two spaces for both slashes
            for next_ch in chars.by_ref() {
              if next_ch == '\n' {
                result.push(next_ch);
                break;
              }
              result.push(' ');
            }
          }
          Some(other) => result.push(other),
          None => break,
        }
      }
      '"' => {
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
      }

      _ => result.push(ch),
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
