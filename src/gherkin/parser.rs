use crate::gherkin::lexer::{self, LineType};

pub fn file(lines: Vec<lexer::Line>) -> Feature {
    let mut current_block: Option<Block> = None;
    let mut current_step: Option<Step> = None;
    let mut blocks: Vec<Block> = vec![];
    let mut initial_lines = vec![];
    for line in lines {
        match line.line_type {
            LineType::BlockStart => {
                if let Some(mut block) = current_block.take() {
                    if let Some(step) = current_step.take() {
                        block.steps.push(step);
                    }
                    blocks.push(block);
                }
                current_block = Some(Block {
                    title_line: line.text,
                    line_number: line.number,
                    steps: vec![],
                });
            }
            LineType::StepStart => {
                if let Some(step) = current_step.take() {
                    if let Some(block) = current_block.as_mut() {
                        block.steps.push(step);
                    }
                }
                current_step = Some(Step {
                    title: first_word_after_trim(&line.text, line.indent.into()).to_string(),
                    lines: vec![line.text],
                })
            }
            LineType::Other => {
                if let Some(step) = current_step.as_mut() {
                    step.lines.push(line.text);
                } else {
                    initial_lines.push(line.text);
                }
            }
        }
    }
    if let Some(step) = current_step.take() {
        if let Some(block) = current_block.as_mut() {
            block.steps.push(step);
        }
    }

    if let Some(block) = current_block.take() {
        blocks.push(block);
    }
    Feature {
        initial_lines,
        blocks,
    }
}

fn first_word_after_trim(text: &str, indentation: usize) -> &str {
    if let Some((_first_word, remainder)) = text[indentation..].split_once(" ") {
        remainder
    } else {
        ""
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Feature {
    pub initial_lines: Vec<String>,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Block {
    pub title_line: String,
    pub line_number: usize,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    /// the textual lines making up this step
    pub lines: Vec<String>,

    /// the relevant title of the step (without Given/When/Then)
    pub title: String,
}

#[cfg(test)]
mod tests {
    use crate::gherkin::parser::first_word_after_trim;

    #[test]
    fn without_first_word() {
        assert_eq!(
            first_word_after_trim("    Given a cucumber", 4),
            "a cucumber",
        )
    }
}
