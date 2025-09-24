use crate::config::Config;
use crate::domain;
use camino::Utf8PathBuf;

pub fn file(file: domain::File, config: &Config, issues: &mut Vec<Issue>) {
    for block in file.blocks {
        check_block(&block, config, issues);
    }
}

fn ordered_block(block: domain::Block, config: &Config, issues: &mut Vec<Issue>) -> domain.Block{
    // order the lines in block the same order as the ones in config
}

pub struct Issue {
    pub file: Utf8PathBuf,
    pub line: usize,
    pub problem: String,
}
