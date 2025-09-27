use crate::config::Config;
use crate::gherkin;
use camino::Utf8Path;

/// provides a copy of the given document with all Gherkin steps sorted the same way as in the given configuration
pub fn file(
  file: gherkin::Document,
  config: &Config,
  filename: &Utf8Path,
  issues: &mut Vec<Issue>,
) -> gherkin::Document {
  let mut new_blocks = Vec::<gherkin::Block>::new();
  for file_block in file.blocks {
    let sorted_block = sort_block(file_block, config, filename, issues);
    new_blocks.push(sorted_block);
  }
  gherkin::Document { blocks: new_blocks }
}

/// provides the given block with all steps sorted according to the given configuration
fn sort_block(
  block: gherkin::Block,
  config: &Config,
  filename: &Utf8Path,
  issues: &mut Vec<Issue>,
) -> gherkin::Block {
  match block {
    gherkin::Block::Sortable(block_steps) => {
      gherkin::Block::Sortable(sort_steps(block_steps, &config.steps, filename, issues))
    }
    gherkin::Block::Static(lines) => gherkin::Block::Static(lines),
  }
}

/// orders the given have_steps to follow the same order as the given config_steps
fn sort_steps(
  unordered_steps: Vec<gherkin::Step>,
  config_steps: &[String],
  filename: &Utf8Path,
  issues: &mut Vec<Issue>,
) -> Vec<gherkin::Step> {
  let mut result = Vec::<gherkin::Step>::with_capacity(unordered_steps.len());
  let mut steps = DeletableSteps::from(unordered_steps);
  for config_step in config_steps {
    let mut extracted = steps.extract(config_step);
    result.append(&mut extracted);
  }
  // report the remaining unextracted steps as unknown steps
  for step in steps.elements() {
    issues.push(Issue {
      line: step.line_no,
      problem: format!(
        "{filename}:{}  unknown step: {}",
        step.line_no + 1,
        step.title
      ),
    });
  }
  result
}

fn matches_config_step(gherkin_step: &gherkin::Step, config_step: &str) -> bool {
  gherkin_step.title.starts_with(config_step)
}

/// a Vec that makes it efficient to delete elements from it
struct DeletableSteps(Vec<Option<gherkin::Step>>);

impl DeletableSteps {
  /// moves all steps from self that match the given config_step
  /// into the given result Vec
  fn extract(&mut self, config_step: &str) -> Vec<gherkin::Step> {
    let mut result = vec![];
    for entry_opt in self.0.iter_mut() {
      if let Some(entry) = &entry_opt
        && matches_config_step(entry, config_step)
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
      let have_steps = sort::sort_steps(
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
      let give_block = gherkin::Block::Sortable(vec![
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
      let want_block = gherkin::Block::Sortable(vec![
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
      let have_block = sort::sort_block(give_block, &config, "test.feature".into(), &mut issues);
      pretty::assert_eq!(have_block, want_block);
    }

    #[test]
    fn unknown_step() {
      let config = Config {
        steps: vec![S("step 1"), S("step 2")],
      };
      let give_block = gherkin::Block::Sortable(vec![
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
      let want_block = gherkin::Block::Sortable(vec![
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
      let have_block = sort::sort_block(give_block, &config, "test.feature".into(), &mut issues);
      pretty::assert_eq!(have_block, want_block);
      let want_issues = vec![Issue {
        line: 1,
        problem: S("test.feature:2  unknown step: step 3"),
      }];
      pretty::assert_eq!(want_issues, issues);
    }
  }

  mod steps_collect {
    use crate::gherkin::Step;
    use crate::sort::DeletableSteps;
    use big_s::S;

    #[test]
    fn some_none() {
      let step_1 = Step {
        title: S("title"),
        lines: vec![],
        line_no: 1,
        indent: 0,
      };
      let give = DeletableSteps(vec![None, Some(step_1.clone())]);
      let have: Vec<_> = give.elements().collect();
      let want = vec![step_1];
      assert_eq!(want, have);
    }

    #[test]
    fn all_none() {
      let give = DeletableSteps(vec![None, None]);
      let have: Vec<_> = give.elements().collect();
      let want = Vec::<Step>::new();
      assert_eq!(want, have);
    }
  }
}
