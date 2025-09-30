use ansi_term::Color::Cyan;
use camino::Utf8PathBuf;

/// UserError happen when the user uses this linter the wrong way.
/// They do not include problems that the linter finds in Gherkin files.
#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
  ConfigFileInvalidRegex {
    file: Utf8PathBuf,
    line: usize,
    message: String,
  },
  ConfigFileNotFound {
    file: Utf8PathBuf,
  },
  ConfigFileRead {
    file: Utf8PathBuf,
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
  IgnoreFileInvalidGlob {
    file: Utf8PathBuf,
    line: usize,
    reason: String,
  },
}

impl UserError {
  /// Provides human-readable descriptions for the various errors variants.
  /// The first result is the actual error message,
  /// the second result is an optional description providing additional details.
  pub fn messages(self) -> (String, Option<String>) {
    match self {
      UserError::ConfigFileInvalidRegex {
        file,
        line,
        message,
      } => (
        format!("{}:{}  invalid regular expression", file, line),
        Some(message),
      ),
      UserError::ConfigFileNotFound { file } => (
        format!("config file ({}) not found", file),
        Some(format!(
          "Please run {} to create the config files.",
          Cyan.paint("cucumber-sort init")
        )),
      ),
      UserError::ConfigFileRead { file, reason } => (
        format!("cannot read configuration file: {reason}"),
        Some(format!("The configuration file has name {}.", file)),
      ),
      UserError::FileRead { file, reason } => (format!("cannot read file {file}: {reason}"), None),
      UserError::FileWrite { file, reason } => {
        (format!("cannot write file {file}: {reason}"), None)
      }
      UserError::IgnoreFileInvalidGlob { file, line, reason } => (
        format!("{}:{}  invalid glob expression", file, line),
        Some(reason),
      ),
    }
  }
}
