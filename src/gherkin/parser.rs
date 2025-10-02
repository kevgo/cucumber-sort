use crate::errors::{Finding, Issue, Result};
use crate::gherkin::lexer::{self, Keyword, LineType};
use camino::Utf8Path;
use std::fmt::{Display, Write};

pub fn file(lines: Vec<lexer::Line>) -> Result<Document> {
  let mut blocks: Vec<Block> = vec![];
  let mut open_block: Option<Block> = None; // the block that is currently being populated
  let mut open_step: Option<Step> = None; // the step that is currently being populated
  let mut docstring_indent = None; // if we are inside a docstring, contains the indentation of that docstring
  for line in lines {
    let new_open_block: Option<Block>; // the new value of open_block at the end of this loop
    let new_open_step: Option<Step>; // the new value of open_step at the end of this loop
    match (&line.line_type, open_block, open_step) {
      (LineType::StepStart { keyword }, Some(Block::Static(lines)), None) => {
        // the first step after a static text block
        blocks.push(Block::Static(lines));
        new_open_block = Some(Block::Sortable(vec![]));
        new_open_step = Some(Step {
          line_no: line.number,
          indent: line.indent_text().to_string(),
          keyword: *keyword,
          title: line.title().to_string(),
          additional_lines: vec![],
        });
      }
      (LineType::StepStart { keyword: _ }, Some(Block::Sortable(steps)), Some(mut step))
        if docstring_indent.is_some() =>
      {
        // part of a docstring that looks like a step
        step.additional_lines.push(line.text);
        new_open_block = Some(Block::Sortable(steps));
        new_open_step = Some(step);
      }
      (LineType::StepStart { keyword }, Some(Block::Sortable(mut steps)), Some(step)) => {
        // a step in the middle of populating a sortable block
        steps.push(step);
        new_open_block = Some(Block::Sortable(steps));
        new_open_step = Some(Step {
          line_no: line.number,
          indent: line.indent_text().to_string(),
          keyword: *keyword,
          title: line.title().to_string(),
          additional_lines: vec![],
        });
      }
      (LineType::DocStringStartStop, Some(Block::Sortable(steps)), Some(mut step)) => {
        if let Some(wrapper_indent) = &docstring_indent {
          if *wrapper_indent == step.indent.len() {
            // we found the closing docstring delimiter
            docstring_indent = None;
          }
        } else {
          // we found a starting docstring delimiter
          docstring_indent = Some(step.indent.len());
        }
        step.additional_lines.push(line.text);
        new_open_block = Some(Block::Sortable(steps));
        new_open_step = Some(step);
      }
      (LineType::Text, None, None) => {
        // the first line of the document
        new_open_block = Some(Block::Static(vec![line.text]));
        new_open_step = None;
      }
      (LineType::Text, Some(Block::Sortable(steps)), Some(mut step)) if docstring_indent.is_some() => {
        // we are inside a docstring, this text line does not start a text block, it is part of the docstring content
        step.additional_lines.push(line.text);
        new_open_block = Some(Block::Sortable(steps));
        new_open_step = Some(step);
      }
      (LineType::Text, Some(Block::Sortable(mut steps)), Some(step)) => {
        // the first static line after a sortable block
        steps.push(step);
        blocks.push(Block::Sortable(steps));
        new_open_block = Some(Block::Static(vec![line.text]));
        new_open_step = None;
      }
      (LineType::Text, Some(Block::Static(mut lines)), None) => {
        // another line of text while populating an unsortable block
        lines.push(line.text);
        new_open_block = Some(Block::Static(lines));
        new_open_step = None;
      }
      (LineType::StepStart { keyword: _ }, None, None) => {
        panic!("a Gherkin document cannot start with a step");
      }
      (LineType::StepStart { keyword: _ }, None, Some(_step)) => {
        panic!("shouldn't have a current_step without a current_block")
      }
      (LineType::StepStart { keyword: _ }, Some(Block::Sortable(_steps)), None) => {
        panic!("shouldn't have an open steps block without a current step")
      }
      (LineType::StepStart { keyword: _ }, Some(Block::Static(_lines)), Some(_step)) => {
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
            result.push(format!("{}{} {}", step.indent, step.keyword, step.title));
            result.extend(step.additional_lines);
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
  pub fn find_mismatching(&self, other: &Lines, filepath: &Utf8Path) -> Vec<Finding> {
    let mut result = vec![];
    for (line_no, (self_text, other_text)) in self.0.iter().zip(other.0.iter()).enumerate() {
      if self_text != other_text {
        result.push(Finding {
          file: filepath.into(),
          line: line_no,
          problem: Issue::UnsortedLine {
            have: self_text.into(),
            want: other_text.into(),
          },
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
  /// the absolute line number inside the document at which this step start
  pub line_no: usize,

  /// the whitespace making up the indentation of this step
  pub indent: String,

  pub keyword: Keyword,

  /// step text without Given/When/Then
  pub title: String,

  /// the textual lines making up this step
  pub additional_lines: Vec<String>,
}

#[cfg(test)]
impl Default for Step {
  fn default() -> Self {
    Self {
      title: Default::default(),
      keyword: Keyword::Given,
      additional_lines: Default::default(),
      indent: Default::default(),
      line_no: Default::default(),
    }
  }
}
