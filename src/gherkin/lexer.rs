use std::io::BufRead;

// the words that lines which start a block can start with
pub const BLOCK_STARTERS: &[&str] = &["Background", "Scenario", "Scenario Outline"];

/// the words that lines which start a step can start with
pub const STEP_STARTERS: &[&str] = &["Given", "When", "Then", "And"];

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
    number: usize,
    /// complete text of the line, as it is in the file
    full_text: String,
    /// how much the line is indented
    indent: Indentation,
    /// line text without preceding whitespace
    trimmed_text: TrimmedLine,
    /// whether this is a Given/When/Then line or not
    line_type: LineType,
}

impl Line {
    fn new(text: String, number: usize) -> Line {
        let (indent, trimmed) = clip_start(&text);
        let line_type = trimmed.line_type();
        Line {
            number,
            full_text: text,
            trimmed_text: trimmed,
            indent: indent,
            line_type,
        }
    }
}

/// provides the number of leading whitespace characters and the text without that leading whitespace
fn clip_start(line: &str) -> (Indentation, TrimmedLine) {
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
enum LineType {
    /// this line starts a block, i.e. "Background", "Scenario", etc
    BlockStart,
    /// this line starts a step, i.e. "Given", "When", "Then", etc
    StepStart,
    /// this line is neither a block or step start
    Other,
}

/// describes how much a line is indented
#[derive(Debug, Eq, PartialEq)]
struct Indentation(usize);

/// a line without the initial whitespace
#[derive(Debug, Eq, PartialEq)]
struct TrimmedLine(String);

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

    mod clip_start {
        use crate::gherkin::lexer::clip_start;

        #[test]
        fn no_indent() {
            let (indent, clipped) = clip_start("text");
            assert_eq!(indent.0, 0);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn two() {
            let (indent, clipped) = clip_start("  text");
            assert_eq!(indent.0, 2);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn four() {
            let (indent, clipped) = clip_start("    text");
            assert_eq!(indent.0, 4);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn only_spaces() {
            let (indent, clipped) = clip_start("    ");
            assert_eq!(indent.0, 4);
            assert_eq!(clipped, "");
        }
    }

    mod line_new {
        use crate::gherkin::lexer::{Indentation, Line, LineType, TrimmedLine};
        use big_s::S;

        #[test]
        fn documentation() {
            let give = "  Some documentation";
            let have = Line::new(S(give), 12);
            let want = Line {
                number: 12,
                full_text: S("  Some documentation"),
                indent: Indentation(2),
                trimmed_text: TrimmedLine::from("Some documentation"),
                line_type: LineType::Other,
            };
            pretty::assert_eq!(have, want);
        }
    }

    mod file {
        use crate::gherkin::lexer::{self, Indentation, Line, LineType, TrimmedLine};
        use big_s::S;
        use std::io::BufReader;

        #[test]
        fn normal() {
            let give = r#"
Feature: test

  A simple example feature file.

  Background:
    Given step 1
    And step 2
    When step 3

  Scenario: result
    Then step 4
    And step 5
"#;
            let bufread = BufReader::new(give[1..].as_bytes());
            let have = lexer::file(bufread);
            let want = vec![
                Line {
                    number: 0,
                    full_text: S("Feature: test"),
                    indent: Indentation(0),
                    trimmed_text: TrimmedLine::from("Feature: test"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 1,
                    full_text: S(""),
                    indent: Indentation(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 2,
                    full_text: S("  A simple example feature file."),
                    indent: Indentation(2),
                    trimmed_text: TrimmedLine::from("A simple example feature file."),
                    line_type: LineType::Other,
                },
                Line {
                    number: 3,
                    full_text: S(""),
                    indent: Indentation(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 4,
                    full_text: S("  Background:"),
                    indent: Indentation(2),
                    trimmed_text: TrimmedLine::from("Background:"),
                    line_type: LineType::BlockStart,
                },
                Line {
                    number: 5,
                    full_text: S("    Given step 1"),
                    indent: Indentation(4),
                    trimmed_text: TrimmedLine::from("Given step 1"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 6,
                    full_text: S("    And step 2"),
                    indent: Indentation(4),
                    trimmed_text: TrimmedLine::from("And step 2"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 7,
                    full_text: S("    When step 3"),
                    indent: Indentation(4),
                    trimmed_text: TrimmedLine::from("When step 3"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 8,
                    full_text: S(""),
                    indent: Indentation(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 9,
                    full_text: S("  Scenario: result"),
                    indent: Indentation(2),
                    trimmed_text: TrimmedLine::from("Scenario: result"),
                    line_type: LineType::BlockStart,
                },
                Line {
                    number: 10,
                    full_text: S("    Then step 4"),
                    indent: Indentation(4),
                    trimmed_text: TrimmedLine::from("Then step 4"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 11,
                    full_text: S("    And step 5"),
                    indent: Indentation(4),
                    trimmed_text: TrimmedLine::from("And step 5"),
                    line_type: LineType::StepStart,
                },
            ];
            pretty::assert_eq!(have, want);
        }
    }
}
