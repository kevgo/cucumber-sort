use crate::errors::{Issue, Result, UserError};
use crate::gherkin::{self, Keyword};
use ansi_term::Color::Cyan;
use camino::Utf8Path;
use regex::Regex;
use std::fs;
use std::io::ErrorKind;

/// the filename of the configuration file
const FILE_NAME: &str = ".cucumbersortrc";

/// Sorter encapsulates the minutiae around checking the order of Gherkin steps.
/// You give it a config file and it sorts Steps for you.
pub struct Sorter {
  pub entries: Vec<Entry>,
}

pub struct Entry {
  regex: Regex,

  /// how often this regex was used in the current invocation of the tool
  used: bool,

  /// where in the config file this regex is defined
  line_no: usize,
}

impl Sorter {
  pub fn load() -> Result<Sorter> {
    match fs::read_to_string(FILE_NAME) {
      Ok(text) => Sorter::parse(&text),
      Err(err) => match err.kind() {
        ErrorKind::NotFound => Err(UserError::ConfigFileNotFound {
          file: FILE_NAME.into(),
        }),
        _ => Err(UserError::ConfigFileRead {
          file: FILE_NAME.into(),
          reason: err.to_string(),
        }),
      },
    }
  }

  /// provides a copy of the given document with all Gherkin steps sorted the same way as in the given configuration
  pub fn sort_file(
    &mut self,
    file: gherkin::Document,
    filename: &Utf8Path,
  ) -> (gherkin::Document, Vec<Issue>) {
    let mut doc_issues = vec![];
    let mut new_blocks = Vec::<gherkin::Block>::new();
    for file_block in file.blocks {
      let (sorted_block, block_issues) = self.sort_block(file_block, filename);
      new_blocks.push(sorted_block);
      doc_issues.extend(block_issues);
    }
    (gherkin::Document { blocks: new_blocks }, doc_issues)
  }

  pub fn unused_regexes(self) -> Vec<String> {
    let mut result = vec![];
    for entry in self.entries {
      if !entry.used {
        result.push(format!(
          "{}:{}  unused regex: {}",
          FILE_NAME,
          entry.line_no,
          Cyan.paint(entry.regex.as_str())
        ));
      }
    }
    result
  }

  fn sort_block(
    &mut self,
    block: gherkin::Block,
    filename: &Utf8Path,
  ) -> (gherkin::Block, Vec<Issue>) {
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
  ) -> (Vec<gherkin::Step>, Vec<Issue>) {
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
      issues.push(Issue {
        line: step.line_no,
        problem: format!(
          "{filename}:{}  unknown step: {}",
          step.line_no + 1,
          Cyan.paint(step.title)
        ),
      });
    }
    (optimize_keywords(result), issues)
  }

  fn parse(text: &str) -> Result<Sorter> {
    let mut steps = vec![];
    for (i, line) in text.lines().enumerate() {
      if line.is_empty() {
        // TODO: also ignore lines starting with # here
        continue;
      }
      match Regex::new(line) {
        Ok(regex) => steps.push(Entry {
          regex,
          used: false,
          line_no: i + 1,
        }),
        Err(err) => {
          return Err(UserError::ConfigFileInvalidRegex {
            file: FILE_NAME.into(),
            line: i,
            message: err.to_string(),
          });
        }
      }
    }
    Ok(Sorter { entries: steps })
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

  mod sort_steps {
    use crate::errors::Issue;
    use crate::gherkin;
    use crate::gherkin::{Keyword, Sorter};
    use ansi_term::Color::Cyan;
    use big_s::S;

    #[test]
    fn already_ordered() {
      let mut sorter = Sorter::parse("step 1\nstep 2\nstep 3").unwrap();
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
      let mut sorter = Sorter::parse("step 1\nstep 2\nstep 3").unwrap();
      let give_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          title: S("step 3"),
          keyword: Keyword::Given,
          additional_lines: vec![],
          line_no: 0,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 2"),
          keyword: Keyword::And,
          additional_lines: vec![],
          line_no: 1,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 1"),
          keyword: Keyword::And,
          additional_lines: vec![],
          line_no: 2,
          indent: S(""),
        },
      ]);
      let want_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          title: S("step 1"),
          keyword: Keyword::Given,
          additional_lines: vec![],
          line_no: 2,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 2"),
          keyword: Keyword::And,
          additional_lines: vec![],
          line_no: 1,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 3"),
          keyword: Keyword::And,
          additional_lines: vec![],
          line_no: 0,
          indent: S(""),
        },
      ]);
      let (have_block, issues) = sorter.sort_block(give_block, "test.feature".into());
      pretty::assert_eq!(want_block, have_block);
      assert!(issues.is_empty());
    }

    #[test]
    fn unknown_step() {
      let mut sorter = Sorter::parse("step 1\nstep 2").unwrap();
      let give_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          title: S("step 2"),
          keyword: Keyword::Given,
          additional_lines: vec![],
          line_no: 0,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 3"),
          keyword: Keyword::Given,
          additional_lines: vec![],
          line_no: 1,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 1"),
          keyword: Keyword::Given,
          additional_lines: vec![],
          line_no: 2,
          indent: S(""),
        },
      ]);
      let want_block = gherkin::Block::Sortable(vec![
        gherkin::Step {
          title: S("step 1"),
          keyword: Keyword::Given,
          additional_lines: vec![],
          line_no: 2,
          indent: S(""),
        },
        gherkin::Step {
          title: S("step 2"),
          keyword: Keyword::And,
          additional_lines: vec![],
          line_no: 0,
          indent: S(""),
        },
      ]);
      let (have_block, issues) = sorter.sort_block(give_block, "test.feature".into());
      pretty::assert_eq!(want_block, have_block);
      let want_issues = vec![Issue {
        line: 1,
        problem: format!("test.feature:2  unknown step: {}", Cyan.paint("step 3")),
      }];
      pretty::assert_eq!(want_issues, issues);
    }
  }
}
