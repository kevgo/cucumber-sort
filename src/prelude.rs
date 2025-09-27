//! stuff that is used in pretty much every file of this crate

use crate::cmd::available_commands;
use camino::Utf8PathBuf;
use core::fmt::Display;

/// UserError are errors that the user makes around using this linter the wrong way.
/// They do not include problems that the linter finds in Gherkin files.
#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
  CannotReadConfigFile { file: Utf8PathBuf, reason: String },
  CannotReadFile { file: Utf8PathBuf, reason: String },
  CannotWriteFile { file: Utf8PathBuf, reason: String },
  UnknownCommand(String),
}

impl Display for UserError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      UserError::CannotReadConfigFile {
        file: filename,
        reason,
      } => {
        write!(f, "cannot read configuration file ({filename}): {reason}")
      }
      UserError::CannotReadFile { file, reason } => {
        write!(f, "cannot read file {file}: {reason}")
      }
      UserError::CannotWriteFile { file, reason } => {
        write!(f, "cannot write file {file}: {reason}")
      }
      UserError::UnknownCommand(cmd) => {
        write!(f, "unknown command: {cmd}\n\n{}", available_commands())
      }
    }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
