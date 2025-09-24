use crate::config::Config;
use crate::domain;
use camino::Utf8PathBuf;

pub fn file(file: domain::File, config: &Config, issues: &mut Vec<Issue>) {
    for block in file.blocks {
        check_block(&block, config, issues);
    }
}

fn check_block(block: &domain::Block, config: &Config, issues: &mut Vec<Issue>) {
    // ensure the lines in block have the same order as the ones in config
}

pub struct Issue {
    pub file: Utf8PathBuf,
    pub line: usize,
    pub problem: String,
}
