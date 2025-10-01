use crate::regex::insert_regex_placeholders;
use ansi_term::Color::{Green, Red};
use camino::Utf8PathBuf;
use std::cmp::Ordering;
use std::fmt::Display;

/// AppFindings are issues with .feature files that the app finds.
/// Problems where the user calls the app wrong are tracked in `UserError`.
#[derive(Debug, Eq, PartialEq)]
pub struct AppFinding {
  pub file: Utf8PathBuf,
  /// 0-based line number
  pub line: usize,
  pub problem: Issue,
}

impl Display for AppFinding {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.problem {
      Issue::UndefinedStep(text) => {
        write!(
          f,
          "{}:{}  unknown step: {}",
          self.file,
          self.line + 1,
          insert_regex_placeholders(text)
        )
      }
      Issue::UnsortedLine { have, want } => {
        write!(
          f,
          "{}:{}  expected {} but found {}",
          self.file,
          self.line + 1,
          Green.paint(want.trim()),
          Red.paint(have.trim())
        )
      }
      Issue::UnusedRegex(text) => {
        write!(f, "{}:{}  unused regex: {text}", self.file, self.line + 1)
      }
    }
  }
}

impl Ord for AppFinding {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.file.cmp(&other.file) {
      Ordering::Equal => self.line.cmp(&other.line),
      other => other,
    }
  }
}

impl PartialOrd for AppFinding {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Issue {
  /// a .feature file contains a step that doesn't match any regexes in the config file
  UndefinedStep(String),

  /// a line in a .feature file does not contain text that the sorted version has
  UnsortedLine { have: String, want: String },

  /// the config file contains a regex that isn't used in any .feature file
  UnusedRegex(String),
}

#[cfg(test)]
mod tests {
  use crate::errors::{AppFinding, Issue};
  use big_s::S;

  #[test]
  fn ordering() {
    let mut give = vec![
      AppFinding {
        file: "two.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "one.feature".into(),
        line: 2,
        problem: Issue::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "one.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
    ];
    let want = vec![
      AppFinding {
        file: "one.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "one.feature".into(),
        line: 2,
        problem: Issue::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "two.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
    ];
    give.sort();
    pretty::assert_eq!(want, give);
  }
}
