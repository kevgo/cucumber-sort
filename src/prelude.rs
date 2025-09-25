//! stuff that is used in pretty much every file of this crate

use crate::cmd::available_commands;
use camino::Utf8PathBuf;
use core::fmt::Display;

/// errors that the user can do something about,
/// and which the app should therefore display to them
#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
    CannotReadConfigFile { file: Utf8PathBuf, reason: String },
    CannotReadFile { file: Utf8PathBuf, reason: String },
    GherkinBlockContainsNonExecutableLine { file: Utf8PathBuf, line: usize },
    StepOutsideOfBlock { file: Utf8PathBuf, line: usize },
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
            UserError::CannotReadFile {
                file: filename,
                reason,
            } => {
                write!(f, "cannot read file {}: {}", filename, reason)
            }
            UserError::GherkinBlockContainsNonExecutableLine {
                file: filename,
                line,
            } => {
                write!(
                    f,
                    "{filename}:{line}  Gherkin block contains non-executable line",
                )
            }
            UserError::StepOutsideOfBlock { file, line } => {
                write!(f, "{file}:{line}  Gherkin step outside of a block")
            }
            UserError::UnknownCommand(cmd) => {
                write!(f, "unknown command: {cmd}\n\n{}", available_commands())
            }
        }
    }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
