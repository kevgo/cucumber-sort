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

// Removes consecutive unsorted lines from the given findings.
pub fn filter_consecutive_unsorted_lines(findings: Vec<Finding>) -> Vec<Finding> {
  let mut result = vec![];
  let mut prev_was_unsorted_at: Option<(Utf8PathBuf, usize)> = None;
  for finding in findings {
    match &finding.problem {
      Issue::UnsortedLine { .. } => {
        let is_consecutive = if let Some((prev_file, prev_line)) = &prev_was_unsorted_at {
          &finding.file == prev_file && finding.line == prev_line + 1
        } else {
          false
        };
        if is_consecutive {
          prev_was_unsorted_at = Some((finding.file, finding.line));
        } else {
          prev_was_unsorted_at = Some((finding.file.clone(), finding.line));
          result.push(finding);
        }
      }
      _ => {
        prev_was_unsorted_at = None;
        result.push(finding);
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::errors::{Finding, Issue};
  use big_s::S;

  mod filter_consecutive_unsorted_lines {
    use crate::errors::{Finding, Issue};
    use camino::Utf8PathBuf;

    #[test]
    fn consecutive_unsorted_lines() {
      let one = Utf8PathBuf::from("one");
      let give = vec![
        Finding {
          file: one.clone(),
          line: 1,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: one.clone(),
          line: 2,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
      ];
      let want = vec![Finding {
        file: one,
        line: 1,
        problem: Issue::UnsortedLine {
          have: "a".into(),
          want: "b".into(),
        },
      }];
      let have = super::super::filter_consecutive_unsorted_lines(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn consecutive_unsorted_lines_in_different_files() {
      let give = vec![
        Finding {
          file: "one.feature".into(),
          line: 1,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: "two.feature".into(),
          line: 2,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
      ];
      let want = vec![
        Finding {
          file: "one.feature".into(),
          line: 1,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: "two.feature".into(),
          line: 2,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
      ];
      let have = super::super::filter_consecutive_unsorted_lines(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn mixing_unsorted_lines_with_other_issues() {
      let one = Utf8PathBuf::from("one");
      let give = vec![
        Finding {
          file: one.clone(),
          line: 1,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: one.clone(),
          line: 2,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: one.clone(),
          line: 3,
          problem: Issue::UndefinedStep("step".into()),
        },
        Finding {
          file: one.clone(),
          line: 4,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: one.clone(),
          line: 5,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
      ];
      let want = vec![
        Finding {
          file: one.clone(),
          line: 1,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
        Finding {
          file: one.clone(),
          line: 3,
          problem: Issue::UndefinedStep("step".into()),
        },
        Finding {
          file: one,
          line: 4,
          problem: Issue::UnsortedLine {
            have: "a".into(),
            want: "b".into(),
          },
        },
      ];
      let have = super::super::filter_consecutive_unsorted_lines(give);
      pretty::assert_eq!(have, want);
    }
  }

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
