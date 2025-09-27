//! stuff that is used in pretty much every file of this crate

use crate::cmd::available_commands;
use crate::config;
use camino::Utf8PathBuf;

/// UserError happen when the user uses this linter the wrong way.
/// They do not include problems that the linter finds in Gherkin files.
#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
  CannotReadConfigFile { reason: String },
  CannotReadFile { file: Utf8PathBuf, reason: String },
  CannotWriteFile { file: Utf8PathBuf, reason: String },
  UnknownCommand(String),
}

impl UserError {
  /// Provides human-readable descriptions for the various errors variants.
  /// The first result is the actual error message,
  /// the second result is an optional description providing additional details.
  pub fn messages(&self) -> (String, Option<String>) {
    match self {
      UserError::CannotReadConfigFile { reason } => (
        format!("cannot read configuration file: {reason}"),
        Some(format!(
          "The configuration file has name {}.",
          config::FILE_NAME
        )),
      ),
      UserError::CannotReadFile { file, reason } => {
        (format!("cannot read file {file}: {reason}"), None)
      }
      UserError::CannotWriteFile { file, reason } => {
        (format!("cannot write file {file}: {reason}"), None)
      }
      UserError::UnknownCommand(cmd) => (
        format!("unknown command: {cmd}"),
        Some(available_commands().to_string()),
      ),
    }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
