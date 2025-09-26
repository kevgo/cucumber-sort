use crate::gherkin::lexer::{self, LineType};
use crate::prelude::*;
use crate::sort::Issue;
use ansi_term::Color::{Cyan, Red};
use camino::Utf8Path;
use std::fmt::Display;

pub fn file(lines: Vec<lexer::Line>) -> Result<Feature> {
  let mut blocks: Vec<Block> = vec![];
  let mut open_block: Option<Block> = None;
  let mut open_step: Option<Step> = None;
  for line in lines {
    println!("LINE: {}", line.text);
    let new_open_block: Option<Block>;
    let new_open_step: Option<Step>;
    match (&line.line_type, open_block, open_step) {
      (LineType::StepStart, None, None) => {
        new_open_block = Some(Block::Steps(vec![]));
        new_open_step = Some(Step {
          title: line.title().to_string(),
          lines: vec![line.text],
          indent: line.indent,
          line_no: line.number,
        });
      }
      (LineType::StepStart, Some(Block::Text(lines)), None) => {
        blocks.push(Block::Text(lines));
        new_open_block = Some(Block::Steps(vec![]));
        new_open_step = Some(Step {
          title: line.title().to_string(),
          lines: vec![line.text],
          indent: line.indent,
          line_no: line.number,
        });
      }
      (LineType::StepStart, Some(Block::Steps(mut steps)), Some(step)) => {
        steps.push(step);
        new_open_block = Some(Block::Steps(steps));
        new_open_step = Some(Step {
          title: line.title().to_string(),
          lines: vec![line.text],
          indent: line.indent,
          line_no: line.number,
        });
      }
      (LineType::CommentStartStop, Some(Block::Steps(steps)), Some(mut step)) => {
        step.lines.push(line.text);
        new_open_block = Some(Block::Steps(steps));
        new_open_step = Some(step);
      }
      (LineType::Other, None, None) => {
        new_open_block = Some(Block::Text(vec![line.text]));
        new_open_step = None;
      }
      (LineType::Other, Some(Block::Steps(mut steps)), Some(step)) => {
        steps.push(step);
        blocks.push(Block::Steps(steps));
        new_open_block = Some(Block::Text(vec![line.text]));
        new_open_step = None;
      }
      (LineType::Other, Some(Block::Text(mut lines)), None) => {
        lines.push(line.text);
        new_open_block = Some(Block::Text(lines));
        new_open_step = None;
      }
      (LineType::StepStart, None, Some(_step)) => {
        panic!("shouldn't have a current_step without a current_block")
      }
      (LineType::StepStart, Some(Block::Steps(_steps)), None) => {
        panic!("shouldn't have an open steps block without a current step")
      }
      (LineType::StepStart, Some(Block::Text(_lines)), Some(_step)) => {
        panic!("should not have an open step while there is an open text block");
      }
      (LineType::CommentStartStop, None, None) => {
        panic!("should not have a comment start without an open step or block")
      }
      (LineType::CommentStartStop, None, Some(_step)) => {
        panic!("should not have a comment start without an open block")
      }
      (LineType::CommentStartStop, Some(_block), None) => {
        panic!("should not have a docstring start without an open step")
      }
      (LineType::CommentStartStop, Some(Block::Text(_lines)), Some(_step)) => {
        panic!("should not have an opening comment and open step in the middle of a text block")
      }
      (LineType::Other, None, Some(_step)) => {
        panic!("should not have an open step without an open block")
      }
      (LineType::Other, Some(_block), None) => {
        panic!("should not have an open block without an open step")
      }
      (LineType::Other, Some(Block::Text(_lines)), Some(_step)) => {
        panic!("should not have an open step in the middle of populating a text block")
      }
    }
    open_block = new_open_block;
    open_step = new_open_step;
  }
  if let Some(block) = open_block {
    match block {
      Block::Steps(mut steps) => {
        if let Some(step) = open_step {
          steps.push(step);
        }
        blocks.push(Block::Steps(steps));
      }
      Block::Text(lines) => {
        blocks.push(Block::Text(lines));
      }
    }
  }
  Ok(Feature { blocks })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Feature {
  pub blocks: Vec<Block>,
}

impl Feature {
  pub fn lines(self) -> Lines {
    let mut result = vec![];
    for block in self.blocks {
      match block {
        Block::Steps(steps) => {
          for mut step in steps {
            result.append(&mut step.lines);
          }
        }
        Block::Text(mut lines) => {
          result.append(&mut lines);
        }
      }
    }
    Lines(result)
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Lines(Vec<String>);

impl Lines {
  pub fn find_mismatching(&self, other: &Lines, filepath: &Utf8Path, issues: &mut Vec<Issue>) {
    for (line_no, (self_text, other_text)) in self.0.iter().zip(other.0.iter()).enumerate() {
      println!("line: {} {}", self_text, other_text);
      if self_text != other_text {
        issues.push(Issue {
          line: line_no,
          problem: format!(
            "{filepath}:{line_no}  expected {} but found {}",
            Cyan.paint(other_text.trim()),
            Red.paint(self_text.trim())
          ),
        });
      }
    }
  }
}

impl Display for Lines {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for line in &self.0 {
      _ = f.write_str(line);
    }
    Ok(())
  }
}

impl From<Vec<String>> for Lines {
  fn from(value: Vec<String>) -> Self {
    Lines(value)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
  /// steps that need to be sorted
  Steps(Vec<Step>),
  /// non-executable text
  Text(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
  /// the relevant title of the step (without Given/When/Then)
  pub title: String,

  /// the textual lines making up this step
  pub lines: Vec<String>,

  /// the indentation of this step
  pub indent: usize,

  pub line_no: usize,
}
