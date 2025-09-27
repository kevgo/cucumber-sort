use crate::config::Config;
use crate::gherkin;
use camino::Utf8Path;

/// provides a copy of the given File with all Gherkin steps sorted the same way as the given configuration
pub fn file(
  file: gherkin::Feature,
  config: &Config,
  filename: &Utf8Path,
  issues: &mut Vec<Issue>,
) -> gherkin::Feature {
  let mut new_blocks = Vec::<gherkin::Block>::new();
  for file_block in file.blocks {
    new_blocks.push(block(file_block, config, filename, issues));
  }
  gherkin::Feature { blocks: new_blocks }
}

/// provides the given block with all steps sorted according to the given configuration
fn block(
  block: gherkin::Block,
  config: &Config,
  filename: &Utf8Path,
  issues: &mut Vec<Issue>,
) -> gherkin::Block {
  match block {
    gherkin::Block::Steps(block_steps) => {
      gherkin::Block::Steps(steps(block_steps, &config.steps, filename, issues))
    }
    gherkin::Block::Text(lines) => gherkin::Block::Text(lines),
  }
}

/// orders the given have_steps to follow the same order as the given config_steps
fn steps(
  have_steps: Vec<gherkin::Step>,
  config_steps: &[String],
  filename: &Utf8Path,
  issues: &mut Vec<Issue>,
) -> Vec<gherkin::Step> {
  let mut ordered = Vec::<gherkin::Step>::with_capacity(have_steps.len());
  let mut steps = Steps::from(have_steps);
  for config_step in config_steps {
    let mut extracted = steps.extract(config_step);
    ordered.append(&mut extracted);
  }
  // report unknown steps
  for step in steps.elements() {
    issues.push(Issue {
      line: step.line_no,
      problem: format!("{filename}:{}  unknown step: {}", step.line_no, step.title),
    });
  }
  ordered
}

fn matches_config_step(gherkin_step: &gherkin::Step, config_step: &str) -> bool {
  gherkin_step.title.starts_with(config_step)
}

struct Steps(Vec<Option<gherkin::Step>>);

impl Steps {
  /// provides all steps from self that match the given config_step
  /// and removes those steps from self
  fn extract(&mut self, config_step: &str) -> Vec<gherkin::Step> {
    let mut extracted = Vec::<gherkin::Step>::new();
    for entry_opt in self.0.iter_mut() {
      if let Some(entry) = entry_opt.take() {
        if matches_config_step(&entry, config_step) {
          extracted.push(entry);
        } else {
          let _ = entry_opt.insert(entry);
        }
      }
    }
    extracted
  }

  fn elements(self) -> Vec<gherkin::Step> {
    self.0.into_iter().flatten().collect()
  }
}

impl From<Vec<gherkin::Step>> for Steps {
  fn from(value: Vec<gherkin::Step>) -> Self {
    Steps(value.into_iter().map(Some).collect())
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Issue {
  pub line: usize,
  pub problem: String,
}

pub fn sort_issues(issues: &mut [Issue]) {
  issues.sort_by_key(|issue| issue.line);
}

#[cfg(test)]
mod tests {

  mod sort_steps {
    use crate::config::Config;
    use crate::sort::Issue;
    use crate::{gherkin, sort};
    use big_s::S;

    #[test]
    fn already_ordered() {
      let config_steps = vec![S("step 1"), S("step 2"), S("step 3")];
      let give_steps = vec![
        gherkin::Step {
          title: S("step 1"),
          lines: vec![],
          line_no: 0,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 2"),
          lines: vec![],
          line_no: 1,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 3"),
          lines: vec![],
          line_no: 2,
          indent: 0,
        },
      ];
      let want_steps = give_steps.clone();
      let mut issues = vec![];
      let have_steps = sort::steps(
        give_steps,
        &config_steps,
        "test.feature".into(),
        &mut issues,
      );
      assert_eq!(want_steps, have_steps);
    }

    #[test]
    fn unordered() {
      let config = Config {
        steps: vec![S("step 1"), S("step 2"), S("step 3")],
      };
      let give_block = gherkin::Block::Steps(vec![
        gherkin::Step {
          title: S("step 3"),
          lines: vec![],
          line_no: 0,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 2"),
          lines: vec![],
          line_no: 1,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 1"),
          lines: vec![],
          line_no: 2,
          indent: 0,
        },
      ]);
      let want_block = gherkin::Block::Steps(vec![
        gherkin::Step {
          title: S("step 1"),
          lines: vec![],
          line_no: 2,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 2"),
          lines: vec![],
          line_no: 1,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 3"),
          lines: vec![],
          line_no: 0,
          indent: 0,
        },
      ]);
      let mut issues = vec![];
      let have_block = sort::block(give_block, &config, "test.feature".into(), &mut issues);
      pretty::assert_eq!(have_block, want_block);
    }

    #[test]
    fn unknown_step() {
      let config = Config {
        steps: vec![S("step 1"), S("step 2")],
      };
      let give_block = gherkin::Block::Steps(vec![
        gherkin::Step {
          title: S("step 2"),
          lines: vec![],
          line_no: 0,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 3"),
          lines: vec![],
          line_no: 1,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 1"),
          lines: vec![],
          line_no: 2,
          indent: 0,
        },
      ]);
      let want_block = gherkin::Block::Steps(vec![
        gherkin::Step {
          title: S("step 1"),
          lines: vec![],
          line_no: 2,
          indent: 0,
        },
        gherkin::Step {
          title: S("step 2"),
          lines: vec![],
          line_no: 0,
          indent: 0,
        },
      ]);
      let mut issues = vec![];
      let have_block = sort::block(give_block, &config, "test.feature".into(), &mut issues);
      pretty::assert_eq!(have_block, want_block);
      let want_issues = vec![Issue {
        line: 1,
        problem: S("test.feature:1  unknown step: step 3"),
      }];
      pretty::assert_eq!(want_issues, issues);
    }
  }

  mod steps_collect {
    use crate::gherkin::Step;
    use crate::sort::Steps;
    use big_s::S;

    #[test]
    fn some_none() {
      let step_1 = Step {
        title: S("title"),
        lines: vec![],
        line_no: 1,
        indent: 0,
      };
      let give = Steps(vec![None, Some(step_1.clone())]);
      let have = give.elements();
      let want = vec![step_1];
      assert_eq!(want, have);
    }

    #[test]
    fn all_none() {
      let give = Steps(vec![None, None]);
      let have = give.elements();
      let want = Vec::<Step>::new();
      assert_eq!(want, have);
    }
  }
}
