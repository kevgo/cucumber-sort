//! stuff that is used in pretty much every file of this crate

use camino::Utf8PathBuf;
use core::fmt::Display;

/// errors that the user can do something about,
/// and which the app should therefore display to them
#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
    CannotReadFile {
        filename: Utf8PathBuf,
        reason: String,
    },
}

impl Display for UserError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UserError::CannotReadFile { filename, reason } => {
                write!(f, "cannot read file {}: {}", filename, reason)
            }
        }
    }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
