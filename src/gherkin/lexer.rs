use crate::prelude::*;
use std::fmt::Display;
use std::io::BufRead;

/// lexes the given file content
pub fn file(text: impl BufRead) -> Result<Vec<Line>> {
  let mut result = vec![];
  for (i, text_line) in text.lines().enumerate() {
    let line = Line::new(text_line.unwrap(), i)?;
    result.push(line);
  }
  Ok(result)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line {
  /// the line number in the file
  pub number: usize,

  /// complete text of the line, as it is in the file
  pub text: String,

  /// how much the line is indented
  pub indent: usize,

  pub title_start: usize,

  /// whether this is a Given/When/Then line or not
  pub line_type: LineType,
}

impl Line {
  fn new(text: String, number: usize) -> Result<Line> {
    let mut chars = text.char_indices();
    let mut indent = text.len();
    let mut end_of_first_word = text.len();
    let mut title_start = text.len();

    // step 1: find the end of the initial whitespace
    loop {
      let Some((i, c)) = chars.next() else { break };
      if !c.is_whitespace() {
        indent = i;
        break;
      }
    }
    let trimmed_text = &text[indent..];
    if trimmed_text == "\"\"\"" {
      return docstring_line(number, text, indent);
    }

    // step 2: find the end of the first word
    loop {
      let Some((i, c)) = chars.next() else { break };
      if c.is_whitespace() {
        end_of_first_word = i;
        break;
      }
    }
    let first_word = &text[indent..end_of_first_word];
    let Some(keyword) = Keyword::parse(first_word) else {
      return text_line(number, text, indent);
    };

    // step 3: find the beginning of the title
    loop {
      let Some((i, c)) = chars.next() else { break };
      if !c.is_whitespace() {
        title_start = i;
        break;
      }
    }
    Ok(Line {
      number,
      text,
      indent,
      line_type: LineType::StepStart { keyword },
      title_start,
    })
  }

  /// provides the whitespace characters that make up the indentation of this line
  pub fn indent_text(&self) -> &str {
    &self.text[..self.indent]
  }

  pub fn title(&self) -> &str {
    &self.text[self.title_start..]
  }
}

fn text_line(number: usize, text: String, indent: usize) -> Result<Line> {
  Ok(Line {
    number,
    text,
    indent,
    title_start: indent,
    line_type: LineType::Text,
  })
}

fn docstring_line(number: usize, text: String, indent: usize) -> Result<Line> {
  Ok(Line {
    number,
    text,
    indent,
    title_start: indent,
    line_type: LineType::DocStringStartStop,
  })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineType {
  /// the start or stop of a Gherkin docstring
  DocStringStartStop,
  /// the start of a Gherkin step, i.e. "Given", "When", "Then", etc
  StepStart { keyword: Keyword },
  /// static text that shouldn't be sorted
  Text,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Keyword {
  Given,
  When,
  Then,
  And,
}

impl Keyword {
  pub fn parse(text: &str) -> Option<Keyword> {
    match text.to_ascii_lowercase().as_str() {
      "given" => Some(Keyword::Given),
      "when" => Some(Keyword::When),
      "then" => Some(Keyword::Then),
      "and" => Some(Keyword::And),
      _ => None,
    }
  }
}

impl Display for Keyword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let text = match self {
      Keyword::Given => "Given",
      Keyword::When => "When",
      Keyword::Then => "Then",
      Keyword::And => "And",
    };
    f.write_str(text)
  }
}

#[cfg(test)]
mod tests {

  mod line {
    use crate::gherkin::lexer::Line;
    use big_s::S;

    mod new {
      use crate::gherkin::Keyword;
      use crate::gherkin::lexer::{Line, LineType};
      use big_s::S;

      #[test]
      fn empty_line() {
        let have = Line::new(S(""), 12).unwrap();
        assert_eq!(have.indent_text(), "");
        assert_eq!(have.line_type, LineType::Text);
        assert_eq!(have.title(), "");
      }

      #[test]
      fn whitespace_only() {
        let have = Line::new(S("    "), 12).unwrap();
        assert_eq!(have.indent_text(), "    ");
        assert_eq!(have.line_type, LineType::Text);
        assert_eq!(have.title(), "");
      }

      #[test]
      fn no_spaces_and_text() {
        let have = Line::new(S("Feature: test"), 12).unwrap();
        assert_eq!(have.indent_text(), "");
        assert_eq!(have.line_type, LineType::Text);
        assert_eq!(have.title(), "Feature: test");
      }

      #[test]
      fn two_spaces_and_text() {
        let have = Line::new(S("  text"), 12).unwrap();
        assert_eq!(have.indent_text(), "  ");
        assert_eq!(have.line_type, LineType::Text);
        assert_eq!(have.title(), "text");
      }

      #[test]
      fn two_tabs_and_text() {
        let have = Line::new(S("\t\ttext"), 12).unwrap();
        assert_eq!(have.indent_text(), "\t\t");
        assert_eq!(have.line_type, LineType::Text);
        assert_eq!(have.title(), "text");
      }

      #[test]
      fn four_spaces_docstring() {
        let have = Line::new(S("    \"\"\""), 12).unwrap();
        assert_eq!(have.indent_text(), "    ");
        assert_eq!(have.line_type, LineType::DocStringStartStop);
        assert_eq!(have.title(), "\"\"\"");
      }

      #[test]
      fn four_spaces_and_step() {
        let have = Line::new(S("    Given step 1"), 12).unwrap();
        assert_eq!(have.indent_text(), "    ");
        assert_eq!(
          have.line_type,
          LineType::StepStart {
            keyword: Keyword::Given
          }
        );
        assert_eq!(have.title(), "step 1");
      }
    }

    #[test]
    fn title() {
      let line = Line::new(S("    Given a cucumber"), 4).unwrap();
      assert_eq!("a cucumber", line.title());
    }
  }
}
