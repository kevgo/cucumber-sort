use ansi_term::Color::{Green, Red};
use camino::Utf8PathBuf;
use std::cmp::Ordering;
use std::fmt::Display;

/// Findings are issues with .feature files that the app finds.
/// Problems where the user calls the app wrong are tracked in `UserError`.
#[derive(Debug, Eq, PartialEq)]
pub struct Finding {
  pub file: Utf8PathBuf,
  /// 0-based line number
  pub line: usize,
  pub problem: Issue,
}

impl Display for Finding {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.problem {
      Issue::UndefinedStep(text) => {
        write!(f, "{}:{}  unknown step: {}", self.file, self.line + 1, text)
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

impl Ord for Finding {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.file.cmp(&other.file) {
      Ordering::Equal => self.line.cmp(&other.line),
      other => other,
    }
  }
}

impl PartialOrd for Finding {
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
  use crate::errors::{Finding, Issue};
  use big_s::S;

  #[test]
  fn ordering() {
    let mut give = vec![
      Finding {
        file: "two.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
      Finding {
        file: "one.feature".into(),
        line: 2,
        problem: Issue::UndefinedStep(S("step")),
      },
      Finding {
        file: "one.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
    ];
    let want = vec![
      Finding {
        file: "one.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
      Finding {
        file: "one.feature".into(),
        line: 2,
        problem: Issue::UndefinedStep(S("step")),
      },
      Finding {
        file: "two.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step")),
      },
    ];
    give.sort();
    pretty::assert_eq!(want, give);
  }
}
