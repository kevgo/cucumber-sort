use std::io::BufRead;
use std::usize;

// the words that lines which start a block can start with
pub const BLOCK_STARTERS: &[&str] = &["Background:", "Scenario:", "Scenario Outline:"];

/// the words that lines which start a step can start with
pub const STEP_STARTERS: &[&str] = &["Given ", "When ", "Then ", "And "];

/// lexes the given file content
pub fn file(text: impl BufRead) -> Vec<Line> {
    text.lines()
        .into_iter()
        .enumerate()
        .map(|(number, line)| Line::new(line.unwrap(), number))
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    /// the line number in the file
    pub number: usize,

    /// complete text of the line, as it is in the file
    pub full_text: String,

    /// how much the line is indented
    pub indent: Indentation,

    /// whether this is a Given/When/Then line or not
    pub line_type: LineType,
}

impl Line {
    fn new(text: String, number: usize) -> Line {
        let (indent, trimmed) = trim_whitespace_start(&text);
        let line_type = trimmed.line_type();
        Line {
            number,
            full_text: text,
            indent: indent,
            line_type,
        }
    }
}

/// provides the number of leading whitespace characters and the text without that leading whitespace
fn trim_whitespace_start(line: &str) -> (Indentation, TrimmedLine) {
    let mut counter = 0;
    for c in line.chars().into_iter() {
        if c == ' ' || c == '\t' {
            counter += 1;
            continue;
        }
        return (Indentation(counter), TrimmedLine::from(&line[counter..]));
    }
    (Indentation(counter), TrimmedLine::from(&line[counter..]))
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

/// describes how much a line is indented
#[derive(Debug, Eq, PartialEq)]
pub struct Indentation(usize);

impl Indentation {
    #[cfg(test)]
    pub fn new(value: usize) -> Indentation {
        Indentation(value)
    }
}

impl Into<usize> for Indentation {
    fn into(self) -> usize {
        self.0
    }
}

/// a line without the initial whitespace
#[derive(Debug, Eq, PartialEq)]
pub struct TrimmedLine(String);

impl TrimmedLine {
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

    pub fn without_first_word(&self) -> &str {
        if let Some((_first_word, remainder)) = self.0.split_once(" ") {
            remainder
        } else {
            ""
        }
    }
}

impl From<&str> for TrimmedLine {
    fn from(value: &str) -> Self {
        TrimmedLine(value.to_string())
    }
}

impl PartialEq<&str> for TrimmedLine {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

#[cfg(test)]
mod tests {

    mod trim_whitespace_start {
        use crate::gherkin::lexer::trim_whitespace_start;

        #[test]
        fn no_indent() {
            let (indent, clipped) = trim_whitespace_start("text");
            assert_eq!(indent.0, 0);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn two() {
            let (indent, clipped) = trim_whitespace_start("  text");
            assert_eq!(indent.0, 2);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn four() {
            let (indent, clipped) = trim_whitespace_start("    text");
            assert_eq!(indent.0, 4);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn only_spaces() {
            let (indent, clipped) = trim_whitespace_start("    ");
            assert_eq!(indent.0, 4);
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

        #[test]
        fn without_first_word() {
            assert_eq!(
                TrimmedLine::from("Given a cucumber").without_first_word(),
                "a cucumber",
            )
        }
    }

    mod line_new {
        use crate::gherkin::lexer::{Indentation, Line, LineType};
        use big_s::S;

        #[test]
        fn documentation() {
            let give = "  Some documentation";
            let have = Line::new(S(give), 12);
            let want = Line {
                number: 12,
                full_text: S("  Some documentation"),
                indent: Indentation(2),
                line_type: LineType::Other,
            };
            pretty::assert_eq!(have, want);
        }
    }
}
