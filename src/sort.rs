use crate::config::Config;
use crate::domain;
use crate::gherkin;
use camino::Utf8PathBuf;

/// provides a copy of the given File with all Gherkin steps sorted the same way as the given configuration
pub fn file(file: gherkin::File, config: &Config, issues: &mut Vec<Issue>) -> gherkin::File {
    let mut new_blocks = Vec::<gherkin::Block>::new();
    for file_block in file.blocks {
        new_blocks.push(block(file_block, config, issues));
    }
    gherkin::File {
        blocks: new_blocks,
        initial_lines: file.initial_lines,
    }
}

/// provides the given block with all steps sorted according to the given configuration
fn block(block: gherkin::Block, config: &Config, issues: &mut Vec<Issue>) -> gherkin::Block {
    // order the lines in block the same order as the ones in config
    block
}

pub struct Issue {
    pub file: Utf8PathBuf,
    pub line: usize,
    pub problem: String,
}

#[cfg(test)]
mod tests {

    mod ordered_block {
        use big_s::S;

        use crate::config::Config;
        use crate::{gherkin, sort};

        #[test]
        fn already_ordered() {
            let config = Config {
                steps: vec![S("step 1"), S("step 2"), S("step 3")],
            };
            let give_block = gherkin::Block {
                start: 3,
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
            };
            let want_block = give_block.clone();
            let mut issues = vec![];
            let have_block = sort::block(give_block, &config, &mut issues);
            assert_eq!(have_block, want_block);
        }

        #[test]
        fn unordered() {
            // TODO
        }

        #[test]
        fn unknown_step() {
            // TODO
        }
    }
}
