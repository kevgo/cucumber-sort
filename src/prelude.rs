//! stuff that is used in pretty much every file of this crate

use crate::config;
use big_s::S;
use camino::Utf8PathBuf;

/// UserError happen when the user uses this linter the wrong way.
/// They do not include problems that the linter finds in Gherkin files.
#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
  ConfigFileInvalidRegex {
    filepath: Utf8PathBuf,
    line: usize,
    message: String,
  },
  ConfigFileRead {
    reason: String,
  },
  FileRead {
    file: Utf8PathBuf,
    reason: String,
  },
  FileWrite {
    file: Utf8PathBuf,
    reason: String,
  },
  UnknownGherkinKeyword(String),
}

impl UserError {
  /// Provides human-readable descriptions for the various errors variants.
  /// The first result is the actual error message,
  /// the second result is an optional description providing additional details.
  pub fn messages(&self) -> (String, Option<String>) {
    match self {
      UserError::ConfigFileInvalidRegex {
        filepath,
        line,
        message,
      } => (
        format!("{}:{}  invalid regular expression", filepath, line),
        Some(message.into()),
      ),
      UserError::ConfigFileRead { reason } => (
        format!("cannot read configuration file: {reason}"),
        Some(format!(
          "The configuration file has name {}.",
          config::FILE_NAME
        )),
      ),
      UserError::FileRead { file, reason } => (format!("cannot read file {file}: {reason}"), None),
      UserError::FileWrite { file, reason } => {
        (format!("cannot write file {file}: {reason}"), None)
      }
      UserError::UnknownGherkinKeyword(keyword) => (
        format!("unknown Gherkin keyword: {keyword}"),
        Some(S("Allowed keywords are: Given, When, Then, And")),
      ),
    }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
