use crate::gherkin::lexer::{self, LineType};
use crate::prelude::*;
use crate::sort::Issue;
use ansi_term::Color::{Green, Red};
use camino::Utf8Path;
use std::fmt::{Display, Write};

pub fn file(lines: Vec<lexer::Line>) -> Result<Document> {
  let mut blocks: Vec<Block> = vec![];
  let mut open_block: Option<Block> = None; // the block that is currently being populated
  let mut open_step: Option<Step> = None; // the step that is currently being populated
  let mut inside_docstring = false; // whether we are inside a docstring
  for line in lines {
    let new_open_block: Option<Block>; // the new value of open_block at the end of this loop
    let new_open_step: Option<Step>; // the new value of open_step at the end of this loop
    match (&line.line_type, open_block, open_step) {
      (LineType::StepStart, Some(Block::Static(lines)), None) => {
        // the first step after a static text block
        blocks.push(Block::Static(lines));
        new_open_block = Some(Block::Sortable(vec![]));
        new_open_step = Some(Step {
          title: line.title().to_string(),
          lines: vec![line.text],
          indent: line.indent,
          line_no: line.number,
        });
      }
      (LineType::StepStart, Some(Block::Sortable(mut steps)), Some(step)) => {
        // a step in the middle of populating a sortable block
        steps.push(step);
        new_open_block = Some(Block::Sortable(steps));
        new_open_step = Some(Step {
          title: line.title().to_string(),
          lines: vec![line.text],
          indent: line.indent,
          line_no: line.number,
        });
      }
      (LineType::DocStringStartStop, Some(Block::Sortable(steps)), Some(mut step)) => {
        // a docstring start/end while populating a step
        step.lines.push(line.text);
        new_open_block = Some(Block::Sortable(steps));
        new_open_step = Some(step);
        inside_docstring = !inside_docstring
      }
      (LineType::Text, None, None) => {
        // the first line of the document
        new_open_block = Some(Block::Static(vec![line.text]));
        new_open_step = None;
      }
      (LineType::Text, Some(Block::Sortable(mut steps)), Some(mut step)) => {
        if inside_docstring {
          // we are inside a docstring, this line is part of the docstring content
          step.lines.push(line.text);
          new_open_block = Some(Block::Sortable(steps));
          new_open_step = Some(step);
        } else {
          // the first static line after a sortable block
          steps.push(step);
          blocks.push(Block::Sortable(steps));
          new_open_block = Some(Block::Static(vec![line.text]));
          new_open_step = None;
        }
      }
      (LineType::Text, Some(Block::Static(mut lines)), None) => {
        // another line of text while populating an unsortable block
        lines.push(line.text);
        new_open_block = Some(Block::Static(lines));
        new_open_step = None;
      }
      (LineType::StepStart, None, None) => {
        panic!("a Gherkin document cannot start with a step");
      }
      (LineType::StepStart, None, Some(_step)) => {
        panic!("shouldn't have a current_step without a current_block")
      }
      (LineType::StepStart, Some(Block::Sortable(_steps)), None) => {
        panic!("shouldn't have an open steps block without a current step")
      }
      (LineType::StepStart, Some(Block::Static(_lines)), Some(_step)) => {
        panic!("should not have an open step while there is an open text block");
      }
      (LineType::DocStringStartStop, None, None) => {
        panic!("should not have a comment start without an open step or block")
      }
      (LineType::DocStringStartStop, None, Some(_step)) => {
        panic!("should not have a comment start without an open block")
      }
      (LineType::DocStringStartStop, Some(_block), None) => {
        panic!("should not have a docstring start without an open step")
      }
      (LineType::DocStringStartStop, Some(Block::Static(_lines)), Some(_step)) => {
        panic!("should not have an opening comment and open step in the middle of a text block")
      }
      (LineType::Text, None, Some(_step)) => {
        panic!("should not have an open step without an open block")
      }
      (LineType::Text, Some(_block), None) => {
        panic!("should not have an open block without an open step")
      }
      (LineType::Text, Some(Block::Static(_lines)), Some(_step)) => {
        panic!("should not have an open step in the middle of populating a text block")
      }
    }
    open_block = new_open_block;
    open_step = new_open_step;
  }
  if let Some(block) = open_block {
    match block {
      Block::Sortable(mut steps) => {
        if let Some(step) = open_step {
          steps.push(step);
        }
        blocks.push(Block::Sortable(steps));
      }
      Block::Static(lines) => {
        blocks.push(Block::Static(lines));
      }
    }
  }
  Ok(Document { blocks })
}

/// a high-level parsed Gherkin document
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Document {
  pub blocks: Vec<Block>,
}

impl Document {
  pub fn lines(self) -> Lines {
    let mut result = vec![];
    for block in self.blocks {
      match block {
        Block::Sortable(steps) => {
          for step in steps {
            result.extend(step.lines);
          }
        }
        Block::Static(lines) => {
          result.extend(lines);
        }
      }
    }
    Lines(result)
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Lines(Vec<String>);

impl Lines {
  pub fn find_mismatching(&self, other: &Lines, filepath: &Utf8Path) -> Vec<Issue> {
    let mut result = vec![];
    for (line_no, (self_text, other_text)) in self.0.iter().zip(other.0.iter()).enumerate() {
      //   println!("line: {} {}", self_text, other_text);
      if self_text != other_text {
        result.push(Issue {
          line: line_no,
          problem: format!(
            "{filepath}:{}  expected {} but found {}",
            line_no + 1,
            Green.paint(other_text.trim()),
            Red.paint(self_text.trim())
          ),
        });
      }
    }
    result
  }
}

impl Display for Lines {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for line in &self.0 {
      _ = f.write_str(line);
      _ = f.write_char('\n');
    }
    Ok(())
  }
}

impl From<Vec<String>> for Lines {
  fn from(value: Vec<String>) -> Self {
    Lines(value)
  }
}

/// a section of a Gherkin document
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
  /// this block type contains sortable elements, i.e. steps to be sorted
  Sortable(Vec<Step>),
  /// this block type contains non-sortable text
  Static(Vec<String>),
}

/// a Gherkin step, to be sorted by this app
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
  /// the relevant title of the step (without Given/When/Then)
  pub title: String,

  /// the textual lines making up this step
  pub lines: Vec<String>,

  /// the indentation of this step
  pub indent: usize,

  /// the absolute line number inside the document at which this step start
  pub line_no: usize,
}
