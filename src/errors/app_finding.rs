use ansi_term::Color::{Green, Red};
use camino::Utf8PathBuf;
use std::fmt::Display;

/// AppFindings are issues that the app finds when being used correctly.
/// Problems from using the app the wrong way are tracked as `UserError`.
#[derive(Debug, Eq, PartialEq)]
pub struct AppFinding {
  pub file: Utf8PathBuf,
  pub line: usize,
  pub problem: Problem,
}

impl Display for AppFinding {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.problem {
      Problem::UndefinedStep(text) => {
        write!(f, "{}:{}  undefined step: {text}", self.file, self.line)
      }
      Problem::UnexpectedText { have, want } => {
        write!(
          f,
          "{}:{}  expected {} but found {}",
          self.file,
          self.line,
          Green.paint(want),
          Red.paint(have)
        )
      }
    }
  }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Problem {
  /// a .feature file contains a step that doesn't match any regexes in the config file
  UndefinedStep(String),
  UnexpectedText {
    have: String,
    want: String,
  },
}

pub fn sort_issues(issues: &mut [AppFinding]) {
  issues.sort_by_key(|issue| issue.line);
}
