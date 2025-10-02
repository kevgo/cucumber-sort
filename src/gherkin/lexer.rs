use crate::errors::Result;
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

/// Line represents all lexed information about a line of text from a Gherkin file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line {
  /// the line number in the file
  pub number: usize,

  /// complete text of the line, as it is in the file
  pub text: String,

  /// the number of whitespace characters at the beginning of the line
  pub indent: usize,

  /// where in the text the step title (the part after Given/When/Then) starts
  pub title_start: usize,

  pub line_type: LineType,
}

impl Line {
  fn new(text: String, number: usize) -> Result<Line> {
    let mut chars = text.char_indices();

    // step 1: find the end of the initial whitespace
    let mut indent = text.len(); // counts how many whitespace characters this line has at the beginning
    for (i, c) in chars.by_ref() {
      if !c.is_whitespace() {
        indent = i;
        break;
      }
    }
    let trimmed_text = &text[indent..];
    if trimmed_text.is_empty() {
      return text_line(number, text, indent);
    }
    if trimmed_text == "\"\"\"" {
      return text_line(number, text, indent);
    }

    // step 2: find the end of the first word
    let mut end_of_first_word = text.len(); // at which character the first word ends
    for (i, c) in chars.by_ref() {
      if c.is_whitespace() {
        end_of_first_word = i;
        break;
      }
    }
    let first_word = &text[indent..end_of_first_word];
    let Some(keyword) = Keyword::parse(first_word) else {
      return text_line(number, text, indent);
    };

    // step 3: at this point we know we have a step, find the beginning of the title
    let mut title_start = text.len(); // at which character the step title starts
    for (i, c) in chars.by_ref() {
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

  /// provides the title of the step (the part after Given/When/Then)
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineType {
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

  mod line_new {
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
      assert_eq!(have.line_type, LineType::Text);
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
}
