use crate::gherkin::lexer::{self, LineType};
use crate::prelude::*;
use camino::Utf8Path;

pub fn file(lines: Vec<lexer::Line>, filepath: &Utf8Path) -> Result<Feature> {
    let mut blocks: Vec<Block> = vec![];
    let mut current_block: Option<Block> = None;
    let mut current_step: Option<Step> = None;
    for line in lines {
        match line.line_type {
            LineType::BlockStart => {
                if let Some(mut block) = current_block.take() {
                    if let Block::Executable(executable_block) = &mut block {
                        if let Some(step) = current_step.take() {
                            executable_block.steps.push(step);
                        }
                    }
                    blocks.push(block);
                }
                current_block = Some(Block::Executable(ExecutableBlock {
                    title: line.text,
                    line_no: line.number,
                    steps: vec![],
                }));
            }
            LineType::StepStart => {
                if let Some(step) = current_step.take() {
                    if let Some(mut block) = current_block.as_mut() {
                        if let Block::Executable(executable_block) = &mut block {
                            executable_block.steps.push(step);
                        }
                        current_step = Some(Step {
                            title: cut_first_word_after_trim(&line.text, line.indent.into())
                                .to_string(),
                            lines: vec![line.text],
                            line_no: line.number,
                        });
                    } else {
                        return Err(UserError::StepOutsideOfBlock {
                            file: filepath.to_path_buf(),
                            line: line.number,
                        });
                    }
                } else {
                    current_step = Some(Step {
                        title: cut_first_word_after_trim(&line.text, line.indent.into())
                            .to_string(),
                        lines: vec![line.text],
                        line_no: line.number,
                    })
                }
            }
            LineType::Other => {
                if let Some(block) = &mut current_block {
                    match block {
                        Block::Executable(_) => {
                            if let Some(step) = current_step.as_mut() {
                                step.lines.push(line.text);
                            } else {
                                return Err(UserError::GherkinBlockContainsNonExecutableLine {
                                    file: filepath.to_path_buf(),
                                    line: line.number,
                                });
                            }
                        }
                        Block::NonExecutable(non_exec_block) => {
                            non_exec_block.text.push(line.text);
                        }
                    }
                } else {
                    current_block = Some(Block::NonExecutable(NonExecutableBlock {
                        line_no: line.number,
                        text: vec![line.text],
                    }))
                }
            }
        }
    }

    if let Some(step) = current_step {
        if let Some(mut block) = current_block.as_mut() {
            match &mut block {
                Block::Executable(executable_block) => {
                    executable_block.steps.push(step);
                }
                Block::NonExecutable(non_executable_block) => {
                    return Err(UserError::GherkinBlockContainsNonExecutableLine {
                        file: filepath.to_path_buf(),
                        line: non_executable_block.line_no,
                    });
                }
            }
        }
    }

    if let Some(block) = current_block {
        blocks.push(block);
    }
    Ok(Feature { blocks })
}

fn cut_first_word_after_trim(text: &str, indentation: usize) -> &str {
    if let Some((_first_word, remainder)) = text[indentation..].split_once(" ") {
        remainder
    } else {
        ""
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Feature {
    pub blocks: Vec<Block>,
}

impl Feature {
    pub fn lines(self) -> Vec<(usize, String)> {
        let mut result = vec![];
        for block in self.blocks {
            match block {
                Block::Executable(executable_block) => {
                    for step in executable_block.steps {
                        let mut line_no = step.line_no;
                        result.push((line_no, step.title));
                        line_no += 1;
                        for line in step.lines {
                            result.push((line_no, line));
                            line_no += 1;
                        }
                    }
                }
                Block::NonExecutable(non_executable_block) => {
                    let mut line_no = non_executable_block.line_no;
                    for line in non_executable_block.text {
                        result.push((line_no, line));
                        line_no += 1;
                    }
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    /// this block type contains a Gherkin scenario or background
    Executable(ExecutableBlock),
    /// non-executable text
    NonExecutable(NonExecutableBlock),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutableBlock {
    pub title: String,
    pub line_no: usize,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonExecutableBlock {
    pub line_no: usize,
    pub text: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    /// the relevant title of the step (without Given/When/Then)
    pub title: String,

    /// the textual lines making up this step
    pub lines: Vec<String>,

    pub line_no: usize,
}

#[cfg(test)]
mod tests {
    use crate::gherkin::parser;

    #[test]
    fn cut_first_word_after_trim() {
        assert_eq!(
            "a cucumber",
            parser::cut_first_word_after_trim("    Given a cucumber", 4)
        );
    }
}
