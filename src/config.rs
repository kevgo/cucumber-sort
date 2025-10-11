use crate::cli::Flags;
use crate::errors::{AppResult, UserError};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs;
use std::io::ErrorKind;

/// the filename of the configuration file
pub const CONFIG_FILE_NAME: &str = "cucumber-sort.json";

pub fn create() -> AppResult<()> {
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
  pub fn load() -> AppResult<Config> {
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

  pub fn save(&self) -> AppResult<()> {
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

/// Checks if the text contains any single-line comments outside of strings.
/// Returns true if comments are found, false otherwise.
fn has_comments(text: &str) -> bool {
  let mut chars = text.chars().peekable();
  let mut in_string = false;
  let mut escaped = false;
  while let Some(ch) = chars.next() {
    if in_string {
      if escaped {
        escaped = false;
      } else if ch == '\\' {
        escaped = true;
      } else if ch == '"' {
        in_string = false;
      }
    } else {
      match ch {
        '"' => in_string = true,
        '/' if chars.peek() == Some(&'/') => {
          return true;
        }
        _ => {}
      }
    }
  }
  false
}

/// Strips single-line comments from JSON text,
/// replacing them with spaces to preserve line numbers for error reporting.
/// Returns a Cow to avoid allocation when no comments are present.
fn strip_comments(text: &str) -> Cow<'_, str> {
  // If no comments found, return borrowed reference
  if !has_comments(text) {
    return Cow::Borrowed(text);
  }

  // Build the result with comments stripped
  let mut result = String::with_capacity(text.len());
  let mut chars = text.chars().peekable();
  let mut in_string = false;
  let mut escaped = false;

  while let Some(ch) = chars.next() {
    if in_string {
      result.push(ch);
      if escaped {
        escaped = false;
      } else if ch == '\\' {
        escaped = true;
      } else if ch == '"' {
        in_string = false;
      }
    } else {
      match ch {
        '"' => {
          result.push(ch);
          in_string = true;
        }
        '/' if chars.peek() == Some(&'/') => {
          chars.next(); // consume second '/'
          result.push_str("  ");
          for c in chars.by_ref() {
            if c == '\n' {
              result.push(c);
              break;
            }
            result.push(' ');
          }
        }
        _ => result.push(ch),
      }
    }
  }
  Cow::Owned(result)
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
    fn double_backslash() {
      let give = r#"{"pattern": "\\\\"}"#;
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
