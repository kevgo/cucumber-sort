use crate::config::Config;
use crate::gherkin;
use camino::Utf8PathBuf;

/// provides a copy of the given File with all Gherkin steps sorted the same way as the given configuration
pub fn file(file: gherkin::Feature, config: &Config, issues: &mut Vec<Issue>) -> gherkin::Feature {
    let mut new_blocks = Vec::<gherkin::Block>::new();
    for file_block in file.blocks {
        new_blocks.push(block(file_block, config, issues));
    }
    gherkin::Feature { blocks: new_blocks }
}

/// provides the given block with all steps sorted according to the given configuration
fn block(block: gherkin::Block, config: &Config, issues: &mut Vec<Issue>) -> gherkin::Block {
    match block {
        gherkin::Block::Executable(executable_block) => {
            gherkin::Block::Executable(gherkin::ExecutableBlock {
                title: executable_block.title,
                line_no: executable_block.line_no,
                steps: steps(executable_block.steps, &config.steps, issues),
            })
        }
        gherkin::Block::NonExecutable(non_executable_block) => {
            return gherkin::Block::NonExecutable(non_executable_block);
        }
    }
}

fn steps(
    have_steps: Vec<gherkin::Step>,
    config_steps: &[String],
    issues: &mut Vec<Issue>,
) -> Vec<gherkin::Step> {
    have_steps
}

pub struct Issue {
    pub file: Utf8PathBuf,
    pub line: usize,
    pub problem: String,
}

#[cfg(test)]
mod tests {

    mod sort_steps {
        use crate::config::Config;
        use crate::gherkin::ExecutableBlock;
        use crate::{gherkin, sort};
        use big_s::S;

        #[test]
        fn already_ordered() {
            let config_steps = vec![S("step 1"), S("step 2"), S("step 3")];
            let give_steps = vec![
                gherkin::Step {
                    title: S("step 1"),
                    lines: vec![],
                },
                gherkin::Step {
                    title: S("step 2"),
                    lines: vec![],
                },
                gherkin::Step {
                    title: S("step 3"),
                    lines: vec![],
                },
            ];
            let want_steps = give_steps.clone();
            let mut issues = vec![];
            let have_steps = sort::steps(give_steps, &config_steps, &mut issues);
            assert_eq!(want_steps, have_steps);
        }

        #[test]
        fn unordered() {
            let config = Config {
                steps: vec![S("step 1"), S("step 2"), S("step 3")],
            };
            let give_block = gherkin::Block::Executable(ExecutableBlock {
                title: S("Scenario: test"),
                line_no: 3,
                steps: vec![
                    gherkin::Step {
                        title: S("step 3"),
                        lines: vec![],
                    },
                    gherkin::Step {
                        title: S("step 2"),
                        lines: vec![],
                    },
                    gherkin::Step {
                        title: S("step 1"),
                        lines: vec![],
                    },
                ],
            });
            let want_block = gherkin::Block::Executable(ExecutableBlock {
                title: S("Scenario: test"),
                line_no: 3,
                steps: vec![
                    gherkin::Step {
                        title: S("step 1"),
                        lines: vec![],
                    },
                    gherkin::Step {
                        title: S("step 2"),
                        lines: vec![],
                    },
                    gherkin::Step {
                        title: S("step 3"),
                        lines: vec![],
                    },
                ],
            });
            let mut issues = vec![];
            let have_block = sort::block(give_block, &config, &mut issues);
            pretty::assert_eq!(have_block, want_block);
        }

        #[test]
        fn unknown_step() {
            // TODO
        }
    }
}
