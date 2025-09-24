use crate::prelude::*;
use std::io::BufRead;

const STEP_STARTS: &[&str] = &["Given", "When", "Then", "And"];

pub struct File {
    blocks: Vec<Block>,
}

#[derive(Default)]
pub struct Block {
    start: usize,
    lines: Vec<String>,
}

pub fn gherkin(text: impl BufRead) -> Result<File> {
    let mut blocks = Vec::<Block>::new();
    let mut current_block = Block::default();
    for (number, line) in text.lines().into_iter().enumerate() {
        let line = line.unwrap();
        if is_step(&line) {
            if current_block.lines.is_empty() {
                current_block.start = number;
            }
            current_block.lines.push(line);
        } else {
            if !current_block.lines.is_empty() {
                blocks.push(current_block);
                current_block = Block::default();
            }
        }
    }
    if !current_block.lines.is_empty() {
        blocks.push(current_block);
    }
    Ok(File { blocks })
}

fn is_step(line: &str) -> bool {
    let trimmed = line.trim();
    STEP_STARTS.iter().any(|want| trimmed.starts_with(want))
}
