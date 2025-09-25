use crate::gherkin::lexer::{self, LineType};

pub fn file(lines: Vec<lexer::Line>) -> Feature {
    let mut current_block: Option<Block> = None;
    let mut current_step: Option<Step> = None;
    let mut blocks: Vec<Block> = vec![];
    let mut initial_lines = vec![];
    for line in lines {
        match line.line_type {
            LineType::BlockStart => {
                if let Some(mut block) = current_block {
                    if let Some(step) = current_step {
                        block.steps.push(step);
                    }
                    blocks.push(block);
                }
                current_block = Some(Block {
                    start_line: line.number,
                    steps: vec![],
                });
                current_step = None
            }
            LineType::StepStart => {
                if let Some(step) = current_step {
                    if let Some(mut block) = current_block {
                        block.steps.push(step);
                        current_block = Some(block);
                    }
                }
                current_step = Some(Step {
                    title: line.trimmed_text.into(),
                    lines: vec![line.full_text],
                })
            }
            LineType::Other => {
                if let Some(mut step) = current_step {
                    step.lines.push(line.full_text);
                    current_step = Some(step);
                } else {
                    initial_lines.push(line.full_text);
                }
            }
        }
    }
    if let Some(block) = current_block {
        blocks.push(block);
    }
    Feature {
        initial_lines: initial_lines,
        blocks: blocks,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Feature {
    pub initial_lines: Vec<String>,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Block {
    pub start_line: usize,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    /// the textual lines making up this step
    pub lines: Vec<String>,

    /// the relevant title of the step (without Given/When/Then)
    pub title: String,
}
