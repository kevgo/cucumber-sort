use std::io::BufRead;

// the words that lines which start a block can start with
pub const BLOCK_STARTERS: &[&str] = &["Background:", "Scenario:", "Scenario Outline:"];

/// the words that lines which start a step can start with
pub const STEP_STARTERS: &[&str] = &["Given ", "When ", "Then ", "And "];

/// lexes the given file content
pub fn file(text: impl BufRead) -> Vec<Line> {
  text
    .lines()
    .enumerate()
    .map(|(number, line)| Line::new(line.unwrap(), number))
    .collect()
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
  /// the line number in the file
  pub number: usize,

  /// complete text of the line, as it is in the file
  pub text: String,

  /// how much the line is indented
  pub indent: usize,

  /// whether this is a Given/When/Then line or not
  pub line_type: LineType,
}

impl Line {
  fn new(text: String, number: usize) -> Line {
    let (indent, trimmed) = trim_initial_whitespace(&text);
    let line_type = trimmed.line_type();
    Line {
      number,
      text,
      indent,
      line_type,
    }
  }
}

/// provides the number of leading whitespace characters and the text without that leading whitespace
fn trim_initial_whitespace<'a>(line: &'a str) -> (usize, TrimmedLine<'a>) {
  let mut counter = 0;
  for c in line.chars() {
    if c == ' ' || c == '\t' {
      counter += 1;
      continue;
    }
    return (counter, TrimmedLine::from(&line[counter..]));
  }
  (counter, TrimmedLine::from(&line[counter..]))
}

#[derive(Debug, Eq, PartialEq)]
pub enum LineType {
  /// this line starts a block, i.e. "Background", "Scenario", etc
  BlockStart,
  /// this line starts a step, i.e. "Given", "When", "Then", etc
  StepStart,
  /// this line is neither a block or step start
  Other,
}

/// a line without the initial whitespace
#[derive(Debug, Eq, PartialEq)]
pub struct TrimmedLine<'a>(&'a str);

impl<'a> TrimmedLine<'a> {
  fn line_type(&self) -> LineType {
    if self.is_block_start() {
      LineType::BlockStart
    } else if self.is_step_start() {
      LineType::StepStart
    } else {
      LineType::Other
    }
  }

  fn is_block_start(&self) -> bool {
    BLOCK_STARTERS.iter().any(|word| self.0.starts_with(word))
  }

  fn is_step_start(&self) -> bool {
    STEP_STARTERS.iter().any(|word| self.0.starts_with(word))
  }
}

impl<'a> From<&'a str> for TrimmedLine<'a> {
  fn from(value: &'a str) -> Self {
    TrimmedLine(value)
  }
}

impl<'a> PartialEq<&str> for TrimmedLine<'a> {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

#[cfg(test)]
mod tests {

  mod trim_whitespace_start {
    use crate::gherkin::lexer::trim_initial_whitespace;

    #[test]
    fn no_indent() {
      let (indent, clipped) = trim_initial_whitespace("text");
      assert_eq!(indent, 0);
      assert_eq!(clipped, "text");
    }

    #[test]
    fn two() {
      let (indent, clipped) = trim_initial_whitespace("  text");
      assert_eq!(indent, 2);
      assert_eq!(clipped, "text");
    }

    #[test]
    fn four() {
      let (indent, clipped) = trim_initial_whitespace("    text");
      assert_eq!(indent, 4);
      assert_eq!(clipped, "text");
    }

    #[test]
    fn only_spaces() {
      let (indent, clipped) = trim_initial_whitespace("    ");
      assert_eq!(indent, 4);
      assert_eq!(clipped, "");
    }
  }

  mod trimmed_line {
    use crate::gherkin::lexer::{LineType, TrimmedLine};

    #[test]
    fn is_block_start() {
      assert!(TrimmedLine::from("Background:").is_block_start());
      assert!(TrimmedLine::from("Scenario: result").is_block_start());
      assert!(TrimmedLine::from("Scenario Outline: test").is_block_start());
      assert!(!TrimmedLine::from("Examples:").is_block_start());
    }

    #[test]
    fn is_step_start() {
      assert!(TrimmedLine::from("Given a cucumber").is_step_start());
      assert!(TrimmedLine::from("When I eat it").is_step_start());
      assert!(TrimmedLine::from("Then its gone").is_step_start());
      assert!(TrimmedLine::from("And I am happy").is_step_start());
      assert!(!TrimmedLine::from("Other text").is_step_start());
    }

    #[test]
    fn line_type() {
      assert_eq!(
        TrimmedLine::from("Given a cucumber").line_type(),
        LineType::StepStart
      );
      assert_eq!(
        TrimmedLine::from("Scenario: test").line_type(),
        LineType::BlockStart
      );
      assert_eq!(
        TrimmedLine::from("Feature: test").line_type(),
        LineType::Other
      );
    }
  }

  mod line_new {
    use crate::gherkin::lexer::{Line, LineType};
    use big_s::S;

    #[test]
    fn documentation() {
      let give = "  Some documentation";
      let have = Line::new(S(give), 12);
      let want = Line {
        number: 12,
        text: S("  Some documentation"),
        indent: 2,
        line_type: LineType::Other,
      };
      pretty::assert_eq!(have, want);
    }
  }
}
