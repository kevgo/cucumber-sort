use ansi_term::Color::{Green, Red};
use camino::Utf8PathBuf;
use std::cmp::Ordering;
use std::fmt::Display;

/// AppFindings are issues that the app finds when being used correctly.
/// Problems from using the app the wrong way are tracked as `UserError`.
#[derive(Debug, Eq, PartialEq, PartialOrd)]
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
      Problem::UnsortedLine { have, want } => {
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

impl Ord for AppFinding {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.file.cmp(&other.file) {
      Ordering::Equal => self.line.cmp(&other.line),
      other => other,
    }
  }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Problem {
  /// a .feature file contains a step that doesn't match any regexes in the config file
  UndefinedStep(String),

  /// a line in a .feature file does not contain text that the sorted version has
  UnsortedLine { have: String, want: String },
}

#[cfg(test)]
mod tests {
  use crate::errors::{AppFinding, Problem};
  use big_s::S;

  #[test]
  fn ordering() {
    let mut give = vec![
      AppFinding {
        file: "two.feature".into(),
        line: 1,
        problem: Problem::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "one.feature".into(),
        line: 2,
        problem: Problem::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "one.feature".into(),
        line: 1,
        problem: Problem::UndefinedStep(S("step")),
      },
    ];
    let want = vec![
      AppFinding {
        file: "one.feature".into(),
        line: 1,
        problem: Problem::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "one.feature".into(),
        line: 2,
        problem: Problem::UndefinedStep(S("step")),
      },
      AppFinding {
        file: "two.feature".into(),
        line: 1,
        problem: Problem::UndefinedStep(S("step")),
      },
    ];
    give.sort();
    pretty::assert_eq!(want, give);
  }
}
