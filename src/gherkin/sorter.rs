use crate::config::{Config, StepPattern};
use crate::errors::{AppResult, Finding, Issue, UserError};
use crate::gherkin::{self, Keyword};
use crate::regex::make_regex;
use camino::Utf8Path;
use regex::Regex;
use std::cell::Cell;

/// Sorter encapsulates the minutiae around checking the order of Gherkin steps.
pub struct Sorter {
  /// the lines from the "steps" section in the config file
  pub entries: Vec<Entry>,
}

/// an line from the "steps" section of the config file
pub struct Entry {
  /// the regexes that are defined on this line
  regexes: Vec<TrackedRegex>,

  /// where in the config this line exists, 0-based
  line: usize,
}

pub struct TrackedRegex {
  regex: Regex,

  /// whether this regex was used in the current invocation of the tool
  used: Cell<bool>,
}

impl TrackedRegex {
  pub fn new(regex: Regex) -> TrackedRegex {
    TrackedRegex {
      regex,
      used: Cell::new(false),
    }
  }
}

impl TryFrom<&str> for TrackedRegex {
  type Error = regex::Error;

  fn try_from(text: &str) -> std::result::Result<Self, Self::Error> {
    Ok(TrackedRegex::new(Regex::new(text)?))
  }
}

impl Sorter {
  /// provides a copy of the given document with all Gherkin steps sorted the same way as in the given configuration
  pub fn sort_file(
    &mut self,
    file: gherkin::Document,
    filename: &Utf8Path,
  ) -> (gherkin::Document, Vec<Finding>) {
    let mut doc_issues = vec![];
    let mut new_blocks = Vec::<gherkin::Block>::with_capacity(file.blocks.len());
    for file_block in file.blocks {
      let (sorted_block, block_issues) = self.sort_block(file_block, filename);
      new_blocks.push(sorted_block);
      doc_issues.extend(block_issues);
    }
    (gherkin::Document { blocks: new_blocks }, doc_issues)
  }

  pub fn unused_regexes(&self) -> Vec<Finding> {
    let mut result = vec![];
    for entry in &self.entries {
      for used_regex in &entry.regexes {
        if !used_regex.used.get() {
          result.push(Finding {
            file: crate::config::CONFIG_FILE_NAME.into(),
            line: entry.line,
            problem: Issue::UnusedRegex(used_regex.regex.to_string()),
          });
        }
      }
    }
    result
  }

  fn sort_block(
    &mut self,
    block: gherkin::Block,
    filename: &Utf8Path,
  ) -> (gherkin::Block, Vec<Finding>) {
    match block {
      gherkin::Block::Sortable(block_steps) => {
        let (sorted_steps, issues) = self.sort_steps(block_steps, filename);
        (gherkin::Block::Sortable(sorted_steps), issues)
      }
      gherkin::Block::Static(lines) => (gherkin::Block::Static(lines), vec![]),
    }
  }

  pub fn sort_steps(
    &mut self,
    unordered_steps: Vec<gherkin::Step>,
    filename: &Utf8Path,
  ) -> (Vec<gherkin::Step>, Vec<Finding>) {
    let mut result = Vec::with_capacity(unordered_steps.len());
    let mut deletable_steps = DeletableSteps::from(deoptimize_keywords(unordered_steps));
    for (entry_idx, entry) in self.entries.iter().enumerate() {
      // step 1: find the Gherkin steps that match the regexes for this config file entry
      let candidates = deletable_steps.find_matches(&entry.regexes);
      // step 2: keep each step from (1) only if no other regex also matches and is longer
      for (candidate_idx, candidate_regex) in candidates {
        if deletable_steps.is_longest_regex(
          candidate_idx,
          candidate_regex,
          &self.entries,
          entry_idx,
        ) {
          // step 3: if current regex is the longest: extract step and mark regex as used
          let extracted = deletable_steps.remove(candidate_idx);
          // mark the regex as used
          for regex in &entry.regexes {
            if regex.regex.as_str() == candidate_regex {
              regex.used.set(true);
            }
          }
          result.push(extracted);
        }
      }
    }
    // report the remaining unextracted steps as unknown steps
    let mut issues = vec![];
    for step in deletable_steps.elements() {
      issues.push(Finding {
        file: filename.into(),
        line: step.line_no,
        problem: Issue::UndefinedStep(step.title),
      });
    }
    (optimize_keywords(result), issues)
  }
}

impl TryFrom<&Config> for Sorter {
  type Error = UserError;

  /// Creates a new Sorter from the JSON configuration
  fn try_from(config: &Config) -> std::result::Result<Self, Self::Error> {
    let mut entries = vec![];
    for (i, step_pattern) in config.steps.iter().enumerate() {
      match step_pattern {
        StepPattern::Single(pattern) => match Regex::new(pattern) {
          Ok(regex) => entries.push(Entry {
            regexes: vec![TrackedRegex::new(regex)],
            line: i,
          }),
          Err(err) => {
            return Err(UserError::ConfigFileInvalidRegex {
              file: crate::config::CONFIG_FILE_NAME.into(),
              line: i,
              message: format!("Invalid regex '{}': {}", pattern, err),
            });
          }
        },
        StepPattern::Group(patterns) => {
          let mut regexes = Vec::with_capacity(patterns.len());
          for pattern in patterns {
            match Regex::new(pattern) {
              Ok(regex) => regexes.push(TrackedRegex::new(regex)),
              Err(err) => {
                return Err(UserError::ConfigFileInvalidRegex {
                  file: crate::config::CONFIG_FILE_NAME.into(),
                  line: i,
                  message: format!("Invalid regex '{}': {}", pattern, err),
                });
              }
            }
          }
          entries.push(Entry { regexes, line: i });
        }
      }
    }
    Ok(Sorter { entries })
  }
}

/// a Vec that makes it efficient to delete elements from it
#[derive(Debug, Eq, PartialEq)]
struct DeletableSteps(Vec<Option<gherkin::Step>>);

impl DeletableSteps {
  fn find_matches<'a>(&self, regexes: &'a [TrackedRegex]) -> Vec<(usize, &'a str)> {
    let mut result = vec![];
    for (index, entry_opt) in self.0.iter().enumerate() {
      if let Some(entry) = &entry_opt {
        for regex in regexes {
          if regex.regex.is_match(&entry.title) {
            result.push((index, regex.regex.as_str()));
            break;
          }
        }
      }
    }
    result
  }

  /// Indicates whether the given regex is the best match for the step at the given index.
  fn is_longest_regex(
    &self,
    candidate_idx: usize,
    candidate_regex: &str,
    all_entries: &[Entry],
    current_entry_idx: usize,
  ) -> bool {
    let Some(step) = &self.0[candidate_idx] else {
      return false;
    };
    for entry in &all_entries[current_entry_idx + 1..] {
      for regex in &entry.regexes {
        let is_longer = regex.regex.as_str().len() > candidate_regex.len();
        if is_longer && regex.regex.is_match(&step.title) {
          // found a longer regex that also matches
          return false;
        }
      }
    }
    true
  }

  /// Removes and returns the step at the given index, marking the matching regex as used
  fn remove(&mut self, index: usize) -> gherkin::Step {
    self.0[index].take().unwrap()
  }

  fn elements(self) -> impl Iterator<Item = gherkin::Step> {
    self.0.into_iter().flatten()
  }
}

impl From<Vec<gherkin::Step>> for DeletableSteps {
  fn from(value: Vec<gherkin::Step>) -> Self {
    DeletableSteps(value.into_iter().map(Some).collect())
  }
}

/// converts Gherkin steps where some are starting with "And" to a form where each one starts with Given/When/Then
fn deoptimize_keywords(steps: Vec<gherkin::Step>) -> Vec<gherkin::Step> {
  let mut result = Vec::with_capacity(steps.len());
  let mut previous_keyword = Keyword::And;
  for mut step in steps {
    if step.keyword == Keyword::And {
      step.keyword = previous_keyword;
    } else {
      previous_keyword = step.keyword;
    }
    result.push(step);
  }
  result
}

/// converts Gherkin steps where each one starts with Given/When/Then to the optimized form where subsequent ones start with And
fn optimize_keywords(steps: Vec<gherkin::Step>) -> Vec<gherkin::Step> {
  let mut result = Vec::with_capacity(steps.len());
  let mut previous_keyword = Keyword::And;
  for mut step in steps {
    if step.keyword == previous_keyword {
      step.keyword = Keyword::And;
    } else {
      previous_keyword = step.keyword;
    };
    result.push(step);
  }
  result
}

/// records the given missing steps in the config file
pub fn store_missing(missings: &[Finding]) -> AppResult<()> {
  if missings.is_empty() {
    return Ok(());
  }
  let mut new_steps = vec![];
  for missing in missings {
    match &missing.problem {
      Issue::UndefinedStep(text) => {
        new_steps.push(make_regex(text));
      }
      Issue::UnsortedLine { have: _, want: _ } => {}
      Issue::UnusedRegex(_) => {}
    }
  }
  if new_steps.is_empty() {
    return Ok(());
  }
  new_steps.sort();
  let mut config = Config::load()?;
  for step in new_steps {
    if !config.unknown_steps.contains(&step) {
      config.unknown_steps.push(step);
    }
  }
  config.unknown_steps.sort();
  config.save()
}

#[cfg(test)]
mod tests {
  use crate::gherkin::{Keyword, Step};
  use big_s::S;

  #[test]
  fn deoptimize_and_optimize_keywords() {
    let steps = vec![
      Step {
        keyword: Keyword::Given,
        title: S("step 1"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::And,
        title: S("step 2"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::And,
        title: S("step 3"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::When,
        title: S("step 4"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::And,
        title: S("step 5"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::Then,
        title: S("step 6"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::And,
        title: S("step 7"),
        ..Step::default()
      },
    ];
    let want_deoptimized = vec![
      Step {
        keyword: Keyword::Given,
        title: S("step 1"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::Given,
        title: S("step 2"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::Given,
        title: S("step 3"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::When,
        title: S("step 4"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::When,
        title: S("step 5"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::Then,
        title: S("step 6"),
        ..Step::default()
      },
      Step {
        keyword: Keyword::Then,
        title: S("step 7"),
        ..Step::default()
      },
    ];
    let have_deoptimized = super::deoptimize_keywords(steps.clone());
    pretty::assert_eq!(want_deoptimized, have_deoptimized);
    let have_optimized = super::optimize_keywords(have_deoptimized);
    pretty::assert_eq!(have_optimized, steps);
  }

  mod from_json_config {
    use crate::config::{Config, StepPattern};
    use crate::gherkin::Sorter;

    #[test]
    fn with_single_steps() {
      let config = Config {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
        ],
        ..Default::default()
      };
      let sorter = Sorter::try_from(&config).unwrap();
      assert_eq!(sorter.entries.len(), 2);
      assert_eq!(sorter.entries[0].regexes.len(), 1);
      assert_eq!(sorter.entries[0].regexes[0].regex.as_str(), "step 1");
      assert_eq!(sorter.entries[1].regexes.len(), 1);
      assert_eq!(sorter.entries[1].regexes[0].regex.as_str(), "step 2");
    }

    #[test]
    fn with_grouped_steps() {
      let config = Config {
        steps: vec![
          StepPattern::Group(vec!["step 1".to_string(), "step 2".to_string()]),
          StepPattern::Single("step 3".to_string()),
        ],
        ..Default::default()
      };
      let sorter = Sorter::try_from(&config).unwrap();
      assert_eq!(sorter.entries.len(), 2);
      assert_eq!(sorter.entries[0].regexes.len(), 2);
      assert_eq!(sorter.entries[0].regexes[0].regex.as_str(), "step 1");
      assert_eq!(sorter.entries[0].regexes[1].regex.as_str(), "step 2");
      assert_eq!(sorter.entries[1].regexes.len(), 1);
      assert_eq!(sorter.entries[1].regexes[0].regex.as_str(), "step 3");
    }

    #[test]
    fn invalid_regex() {
      let config = Config {
        steps: vec![StepPattern::Single("[invalid".to_string())],
        ..Default::default()
      };
      let result = Sorter::try_from(&config);
      assert!(result.is_err());
    }
  }

  mod sort_steps {
    use crate::config::{Config, StepPattern};
    use crate::errors::{Finding, Issue};
    use crate::gherkin;
    use crate::gherkin::{Keyword, Sorter};
    use big_s::S;

    #[test]
    fn already_ordered() {
      let config = Config {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
          StepPattern::Single("step 3".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give_steps = vec![
        gherkin::Step {
          title: S("step 1"),
          line_no: 0,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::When,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 2,
          keyword: Keyword::Then,
          ..Default::default()
        },
      ];
      let want_steps = give_steps.clone();
      let (have_steps, issues) = sorter.sort_steps(give_steps, "test.feature".into());
      assert_eq!(want_steps, have_steps);
      assert!(issues.is_empty());
    }

    #[test]
    fn unordered() {
      let config = Config {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
          StepPattern::Single("step 3".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give = vec![
        gherkin::Step {
          title: S("step 3"),
          line_no: 0,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 1"),
          line_no: 2,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let want = vec![
        gherkin::Step {
          title: S("step 1"),
          line_no: 2,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 0,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let (have_block, issues) = sorter.sort_steps(give, "test.feature".into());
      pretty::assert_eq!(want, have_block);
      assert!(issues.is_empty());
    }

    #[test]
    fn multiple_steps() {
      let config = Config {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Group(vec![S("step 2"), S("step 3")]),
          StepPattern::Single("step 4".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give = vec![
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 0,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 0,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 1"),
          line_no: 2,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let want = vec![
        gherkin::Step {
          title: S("step 1"),
          line_no: 2,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 0,
          indent: S(""),
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 0,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let (have_block, issues) = sorter.sort_steps(give, "test.feature".into());
      pretty::assert_eq!(want, have_block);
      assert!(issues.is_empty());
    }

    #[test]
    fn unknown_step() {
      let config = Config {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give = vec![
        gherkin::Step {
          title: S("step 2"),
          line_no: 0,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 3"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 1"),
          line_no: 2,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let want = vec![
        gherkin::Step {
          title: S("step 1"),
          line_no: 2,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("step 2"),
          line_no: 0,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let (have_block, issues) = sorter.sort_steps(give, "test.feature".into());
      pretty::assert_eq!(want, have_block);
      let want_issues = vec![Finding {
        file: "test.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step 3")),
      }];
      pretty::assert_eq!(want_issues, issues);
    }

    #[test]
    fn multiple_matching_regex() {
      let config = Config {
        steps: vec![
          StepPattern::Single("file".to_string()),
          StepPattern::Single("another step".to_string()),
          StepPattern::Single("file one.txt".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give = vec![
        gherkin::Step {
          title: S("file one.txt"),
          line_no: 0,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("file two.txt"),
          line_no: 1,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("another step"),
          line_no: 2,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let want = vec![
        gherkin::Step {
          title: S("file two.txt"),
          line_no: 1,
          keyword: Keyword::Given,
          ..Default::default()
        },
        gherkin::Step {
          title: S("another step"),
          line_no: 2,
          keyword: Keyword::And,
          ..Default::default()
        },
        gherkin::Step {
          title: S("file one.txt"),
          line_no: 0,
          keyword: Keyword::And,
          ..Default::default()
        },
      ];
      let (have_block, issues) = sorter.sort_steps(give, "test.feature".into());
      pretty::assert_eq!(want, have_block);
      assert!(issues.is_empty());
    }
  }
}
