use crate::prelude::{UserError, *};
use std::io::BufRead;

/// the words that lines which start a step can start with
pub const STEP_STARTERS: &[&str] = &["Given ", "When ", "Then ", "And "];

/// lexes the given file content
pub fn file(text: impl BufRead) -> Result<Vec<Line>> {
  let mut result = vec![];
  let mut docstring_indentation = None;
  for (i, text_line) in text.lines().enumerate() {
    let mut line = Line::new(text_line.unwrap(), i)?;
    if docstring_indentation.is_none() && line.line_type == LineType::DocStringStartStop {
      docstring_indentation = Some(line.indent);
    } else if let Some(indentation) = &docstring_indentation
      && line.line_type == LineType::DocStringStartStop
      && line.indent == *indentation
    {
      docstring_indentation = None
    } else if docstring_indentation.is_some() {
      line.line_type = LineType::Text;
    }
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

  /// whether this is a Given/When/Then line or not
  pub line_type: LineType,
}

impl Line {
  fn new(text: String, number: usize) -> Result<Line> {
    let (indent, trimmed) = trim_initial_whitespace(&text);
    let line_type = trimmed.line_type()?;
    Ok(Line {
      number,
      text,
      indent,
      line_type,
    })
  }

  pub fn title(&self) -> &str {
    if let Some((_first_word, remainder)) = self.text[self.indent..].split_once(" ") {
      remainder
    } else {
      ""
    }
  }
}

/// provides the number of leading whitespace characters and the text without that leading whitespace
fn trim_initial_whitespace<'a>(line: &'a str) -> (usize, TrimmedLine<'a>) {
  for (i, c) in line.char_indices() {
    if c != ' ' && c != '\t' {
      return (i, TrimmedLine::from(&line[i..]));
    }
  }
  // here the line is all whitespace or nothing
  (line.len(), TrimmedLine::from(""))
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

impl TryFrom<&str> for Keyword {
  type Error = UserError;

  fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
    match value.to_ascii_lowercase().as_str() {
      "given" => Ok(Keyword::Given),
      "when" => Ok(Keyword::When),
      "then" => Ok(Keyword::Then),
      "and" => Ok(Keyword::And),
      other => Err(UserError::UnknownGherkinKeyword(other.to_string())),
    }
  }
}

/// a line without the initial whitespace
#[derive(Debug, Eq, PartialEq)]
pub struct TrimmedLine<'a>(&'a str);

impl<'a> TrimmedLine<'a> {
  fn line_type(&self) -> Result<LineType> {
    if self.is_docstring_start() {
      Ok(LineType::DocStringStartStop)
    } else if let Some(keyword) = self.is_step_start()? {
      Ok(LineType::StepStart { keyword })
    } else {
      Ok(LineType::Text)
    }
  }

  fn is_docstring_start(&self) -> bool {
    self.0 == "\"\"\""
  }

  fn is_step_start(&self) -> Result<Option<Keyword>> {
    for starter in STEP_STARTERS {
      if !self.0.starts_with(starter) {
        continue;
      }
      let key_text = &self.0[0..starter.len()];
      let Ok(keyword) = Keyword::try_from(key_text) else {
        return Err(UserError::UnknownGherkinKeyword(key_text.to_string()));
      };
      return Ok(Some(keyword));
    }
    Ok(None)
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

  mod trim_initial_whitespace {
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
    use crate::gherkin::lexer::{Keyword, LineType, TrimmedLine};
    use crate::prelude::UserError;
    use big_s::S;

    #[test]
    fn is_step_start() {
      assert_eq!(
        Ok(Some(Keyword::Given)),
        TrimmedLine::from("Given a cucumber").is_step_start()
      );
      assert_eq!(
        Ok(Some(Keyword::When)),
        TrimmedLine::from("When I eat it").is_step_start()
      );
      assert_eq!(
        Ok(Some(Keyword::Then)),
        TrimmedLine::from("Then its gone").is_step_start()
      );
      assert_eq!(
        Ok(Some(Keyword::And)),
        TrimmedLine::from("And I am happy").is_step_start()
      );
      assert_eq!(
        Err(UserError::UnknownGherkinKeyword(S("Other"))),
        TrimmedLine::from("Other text").is_step_start()
      );
    }

    #[test]
    fn line_type() {
      assert_eq!(
        TrimmedLine::from("Given a cucumber").line_type(),
        Ok(LineType::StepStart {
          keyword: Keyword::Given
        })
      );
      assert_eq!(
        TrimmedLine::from("Feature: test").line_type(),
        Ok(LineType::Text)
      );
      assert_eq!(
        TrimmedLine::from("\"\"\"").line_type(),
        Ok(LineType::DocStringStartStop),
      );
    }
  }

  mod line {
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
        line_type: LineType::Text,
      };
      pretty::assert_eq!(Ok(want), have);
    }

    #[test]
    fn title() {
      let line = Line::new(S("    Given a cucumber"), 4).unwrap();
      assert_eq!("a cucumber", line.title());
    }
  }
}
