use crate::config::Config;
use crate::domain;
use camino::Utf8PathBuf;

/// provides a copy of the given File with all Gherkin steps sorted the same way as the given configuration
pub fn file(file: domain::File, config: &Config, issues: &mut Vec<Issue>) -> domain::File {
    let mut new_blocks = Vec::<domain::Block>::new();
    for file_block in file.blocks {
        new_blocks.push(block(file_block, config, issues));
    }
    domain::File {
        blocks: new_blocks,
        initial_lines: file.initial_lines,
    }
}

/// provides the given block with all steps sorted according to the given configuration
fn block(block: domain::Block, config: &Config, issues: &mut Vec<Issue>) -> domain::Block {
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
        use crate::{domain, sort};

        #[test]
        fn already_ordered() {
            let config = Config {
                steps: vec![S("step 1"), S("step 2"), S("step 3")],
            };
            let give_block = domain::Block {
                start: 3,
                steps: vec![
                    domain::Step {
                        text: S("Given step 1"),
                        title: S("step 1"),
                    },
                    domain::Step {
                        text: S("Given step 2"),
                        title: S("step 2"),
                    },
                    domain::Step {
                        text: S("Given step 3"),
                        title: S("step 3"),
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
