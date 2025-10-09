use crate::config::{JsonConfig, StepPattern};
use crate::errors::{Finding, Issue, Result, UserError};
use crate::gherkin::{self, Keyword};
use crate::regex::make_regex;
use camino::Utf8Path;
use regex::Regex;

/// Sorter encapsulates the minutiae around checking the order of Gherkin steps.
/// You give it a config file and it sorts Steps for you.
pub struct Sorter {
  pub entries: Vec<Entry>,
}

pub struct Entry {
  regex: Regex,

  /// whether this regex was used in the current invocation of the tool
  used: bool,

  /// where in the config this regex is defined, 0-based
  line_no: usize,
}

impl Sorter {
  /// records the given missing steps in the config file
  pub fn store_missing(&self, missings: &[Finding]) -> Result<()> {
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
    new_steps.dedup();

    // Load current config
    let mut config = JsonConfig::load()?;

    // Add new steps to unknown-steps, avoiding duplicates
    for step in new_steps {
      if !config.unknown_steps.contains(&step) {
        config.unknown_steps.push(step);
      }
    }
    config.unknown_steps.sort();

    // Save updated config
    config.save()
  }

  /// provides a copy of the given document with all Gherkin steps sorted the same way as in the given configuration
  pub fn sort_file(
    &mut self,
    file: gherkin::Document,
    filename: &Utf8Path,
  ) -> (gherkin::Document, Vec<Finding>) {
    let mut doc_issues = vec![];
    let mut new_blocks = Vec::<gherkin::Block>::new();
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
      if !entry.used {
        result.push(Finding {
          file: crate::config::CONFIG_FILE_NAME.into(),
          line: entry.line_no,
          problem: Issue::UnusedRegex(entry.regex.to_string()),
        });
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
    let mut result = Vec::<gherkin::Step>::with_capacity(unordered_steps.len());
    let mut deletable_steps = DeletableSteps::from(deoptimize_keywords(unordered_steps));
    for config_step in &mut self.entries {
      let extracted = deletable_steps.extract(&config_step.regex);
      if !extracted.is_empty() {
        config_step.used = true;
      }
      result.extend(extracted);
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

impl TryFrom<&JsonConfig> for Sorter {
  type Error = UserError;

  /// Creates a new Sorter from the JSON configuration
  fn try_from(config: &JsonConfig) -> std::result::Result<Self, Self::Error> {
    let mut entries = vec![];
    for (i, step_pattern) in config.steps.iter().enumerate() {
      match step_pattern {
        StepPattern::Single(pattern) => match Regex::new(pattern) {
          Ok(regex) => entries.push(Entry {
            regex,
            used: false,
            line_no: i,
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
          for pattern in patterns {
            match Regex::new(pattern) {
              Ok(regex) => entries.push(Entry {
                regex,
                used: false,
                line_no: i,
              }),
              Err(err) => {
                return Err(UserError::ConfigFileInvalidRegex {
                  file: crate::config::CONFIG_FILE_NAME.into(),
                  line: i,
                  message: format!("Invalid regex '{}': {}", pattern, err),
                });
              }
            }
          }
        }
      }
    }
    Ok(Sorter { entries })
  }
}

/// a Vec that makes it efficient to delete elements from it
struct DeletableSteps(Vec<Option<gherkin::Step>>);

impl DeletableSteps {
  /// moves all steps from self that match the given config_step
  /// into the given result Vec
  fn extract(&mut self, regex: &Regex) -> Vec<gherkin::Step> {
    let mut result = vec![];
    for entry_opt in self.0.iter_mut() {
      if let Some(entry) = &entry_opt
        && regex.is_match(&entry.title)
      {
        result.push(entry_opt.take().unwrap());
      }
    }
    result
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
    use crate::config::{JsonConfig, StepPattern};
    use crate::gherkin::Sorter;

    #[test]
    fn with_single_steps() {
      let config = JsonConfig {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
        ],
        ..Default::default()
      };
      let sorter = Sorter::try_from(&config).unwrap();
      assert_eq!(sorter.entries.len(), 2);
      assert_eq!(sorter.entries[0].regex.as_str(), "step 1");
      assert_eq!(sorter.entries[1].regex.as_str(), "step 2");
    }

    #[test]
    fn with_grouped_steps() {
      let config = JsonConfig {
        steps: vec![
          StepPattern::Group(vec!["step 1".to_string(), "step 2".to_string()]),
          StepPattern::Single("step 3".to_string()),
        ],
        ..Default::default()
      };
      let sorter = Sorter::try_from(&config).unwrap();
      assert_eq!(sorter.entries.len(), 3);
      assert_eq!(sorter.entries[0].regex.as_str(), "step 1");
      assert_eq!(sorter.entries[1].regex.as_str(), "step 2");
      assert_eq!(sorter.entries[2].regex.as_str(), "step 3");
    }

    #[test]
    fn invalid_regex() {
      let config = JsonConfig {
        steps: vec![StepPattern::Single("[invalid".to_string())],
        ..Default::default()
      };
      let result = Sorter::try_from(&config);
      assert!(result.is_err());
    }
  }

  mod sort_steps {
    use crate::config::{JsonConfig, StepPattern};
    use crate::errors::{Finding, Issue};
    use crate::gherkin;
    use crate::gherkin::{Keyword, Sorter};
    use big_s::S;

    #[test]
    fn already_ordered() {
      let config = JsonConfig {
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
          line_no: 0,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 1"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 1,
          indent: S(""),
          keyword: Keyword::When,
          title: S("step 2"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 2,
          indent: S(""),
          keyword: Keyword::Then,
          title: S("step 3"),
          additional_lines: vec![],
        },
      ];
      let want_steps = give_steps.clone();
      let (have_steps, issues) = sorter.sort_steps(give_steps, "test.feature".into());
      assert_eq!(want_steps, have_steps);
      assert!(issues.is_empty());
    }

    #[test]
    fn unordered() {
      let config = JsonConfig {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
          StepPattern::Single("step 3".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          line_no: 0,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 3"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 1,
          indent: S(""),
          title: S("step 2"),
          keyword: Keyword::And,
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 2,
          indent: S(""),
          keyword: Keyword::And,
          title: S("step 1"),
          additional_lines: vec![],
        },
      ]);
      let want_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          line_no: 2,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 1"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 1,
          indent: S(""),
          keyword: Keyword::And,
          title: S("step 2"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 0,
          indent: S(""),
          keyword: Keyword::And,
          title: S("step 3"),
          additional_lines: vec![],
        },
      ]);
      let (have_block, issues) = sorter.sort_block(give_block, "test.feature".into());
      pretty::assert_eq!(want_block, have_block);
      assert!(issues.is_empty());
    }

    #[test]
    fn unknown_step() {
      let config = JsonConfig {
        steps: vec![
          StepPattern::Single("step 1".to_string()),
          StepPattern::Single("step 2".to_string()),
        ],
        ..Default::default()
      };
      let mut sorter = Sorter::try_from(&config).unwrap();
      let give_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          line_no: 0,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 2"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 1,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 3"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 2,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 1"),
          additional_lines: vec![],
        },
      ]);
      let want_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          line_no: 2,
          indent: S(""),
          keyword: Keyword::Given,
          title: S("step 1"),
          additional_lines: vec![],
        },
        gherkin::Step {
          line_no: 0,
          indent: S(""),
          keyword: Keyword::And,
          title: S("step 2"),
          additional_lines: vec![],
        },
      ]);
      let (have_block, issues) = sorter.sort_block(give_block, "test.feature".into());
      pretty::assert_eq!(want_block, have_block);
      let want_issues = vec![Finding {
        file: "test.feature".into(),
        line: 1,
        problem: Issue::UndefinedStep(S("step 3")),
      }];
      pretty::assert_eq!(want_issues, issues);
    }
  }
}
