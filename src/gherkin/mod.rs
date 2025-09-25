mod lexer;
mod parser;

use crate::prelude::*;
use camino::Utf8Path;
pub use parser::{Block, ExecutableBlock, Feature, Step};
use std::fs;
use std::io::{BufRead, BufReader};

pub fn load(filepath: &Utf8Path) -> Result<parser::Feature> {
  let file_content = fs::File::open(&filepath).map_err(|e| UserError::CannotReadFile {
    file: filepath.to_path_buf(),
    reason: e.to_string(),
  })?;
  file(BufReader::new(file_content), filepath)
}

/// parses the given file content into Gherkin
pub fn file(text: impl BufRead, filepath: &Utf8Path) -> Result<parser::Feature> {
  // step 1: lex the file content into token (lines)
  let lines = lexer::file(text);
  // step 2: parse the tokens (lines) into Gherkin data structures
  parser::file(lines, filepath)
}

#[cfg(test)]
mod tests {

  mod lex_and_parse {
    use crate::gherkin::lexer::{self, Line, LineType};
    use crate::gherkin::parser::{ExecutableBlock, NonExecutableBlock};
    use crate::gherkin::{Block, Step, parser};
    use big_s::S;
    use std::io::BufReader;

    #[test]
    fn multiple_scenarios() {
      // step 1: lex Gherkin source into tokens (in our case Lines)
      let source = r#"
Feature: test

  An example feature file.

  Background:
    Given step 1
    And step 2
    When step 3

  Scenario: result
    Then step 4
    And step 5

  Scenario: undo
    When step 6
    Then step 7
"#;
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..]));
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 1,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 2,
          text: S("  An example feature file."),
          indent: 2,
          line_type: LineType::Other,
        },
        Line {
          number: 3,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 4,
          text: S("  Background:"),
          indent: 2,
          line_type: LineType::BlockStart,
        },
        Line {
          number: 5,
          text: S("    Given step 1"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 6,
          text: S("    And step 2"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 7,
          text: S("    When step 3"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 8,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 9,
          text: S("  Scenario: result"),
          indent: 2,
          line_type: LineType::BlockStart,
        },
        Line {
          number: 10,
          text: S("    Then step 4"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 11,
          text: S("    And step 5"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 12,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 13,
          text: S("  Scenario: undo"),
          indent: 2,
          line_type: LineType::BlockStart,
        },
        Line {
          number: 14,
          text: S("    When step 6"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 15,
          text: S("    Then step 7"),
          indent: 4,
          line_type: LineType::StepStart,
        },
      ];
      pretty::assert_eq!(have_lines, want_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines, "file.feature".into()).unwrap();
      let want_feature = parser::Feature {
        blocks: vec![
          Block::NonExecutable(NonExecutableBlock {
            line_no: 0,
            text: vec![
              S("Feature: test"),
              S(""),
              S("  An example feature file."),
              S(""),
            ],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Background:"),
            line_no: 4,
            steps: vec![
              Step {
                lines: vec![S("    Given step 1")],
                title: S("step 1"),
                line_no: 5,
              },
              Step {
                lines: vec![S("    And step 2")],
                title: S("step 2"),
                line_no: 6,
              },
              Step {
                lines: vec![S("    When step 3"), S("")],
                title: S("step 3"),
                line_no: 7,
              },
            ],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Scenario: result"),
            line_no: 9,
            steps: vec![
              Step {
                lines: vec![S("    Then step 4")],
                title: S("step 4"),
                line_no: 10,
              },
              Step {
                lines: vec![S("    And step 5"), S("")],
                title: S("step 5"),
                line_no: 11,
              },
            ],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Scenario: undo"),
            line_no: 13,
            steps: vec![
              Step {
                lines: vec![S("    When step 6")],
                title: S("step 6"),
                line_no: 14,
              },
              Step {
                lines: vec![S("    Then step 7")],
                title: S("step 7"),
                line_no: 15,
              },
            ],
          }),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);
    }

    #[test]
    fn docstrings() {
      // step 1: lex Gherkin source into tokens (in our case Lines)
      let source = r#"
Feature: test

  Scenario: with docstring
    Given step 1:
      """
      docstring line 1
      docstring line 2
      """
    And step 2
"#;
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..]));
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 1,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 2,
          text: S("  Scenario: with docstring"),
          indent: 2,
          line_type: LineType::BlockStart,
        },
        Line {
          number: 3,
          text: S("    Given step 1:"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 4,
          text: S(r#"      """"#),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 5,
          text: S("      docstring line 1"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 6,
          text: S("      docstring line 2"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 7,
          text: S(r#"      """"#),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 8,
          text: S("    And step 2"),
          indent: 4,
          line_type: LineType::StepStart,
        },
      ];
      pretty::assert_eq!(have_lines, want_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines, "file.feature".into()).unwrap();
      let want_feature = parser::Feature {
        blocks: vec![
          Block::NonExecutable(NonExecutableBlock {
            line_no: 0,
            text: vec![S("Feature: test"), S("")],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Scenario: with docstring"),
            line_no: 2,
            steps: vec![
              Step {
                title: S("step 1:"),
                lines: vec![
                  S("    Given step 1:"),
                  S("      \"\"\""),
                  S("      docstring line 1"),
                  S("      docstring line 2"),
                  S("      \"\"\""),
                ],
                line_no: 3,
              },
              Step {
                title: S("step 2"),
                lines: vec![S("    And step 2")],
                line_no: 8,
              },
            ],
          }),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);
    }

    #[test]
    fn tables() {
      // step 1: lex Gherkin source into tokens (in our case Lines)
      let source = r#"
Feature: test

  Scenario: with table
    Given step 1:
      | HEAD A | HEAD B |
      | row 1A | row 1B |
      | row 2A | row 2B |
    And step 2
"#;
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..]));
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 1,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 2,
          text: S("  Scenario: with table"),
          indent: 2,
          line_type: LineType::BlockStart,
        },
        Line {
          number: 3,
          text: S("    Given step 1:"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 4,
          text: S("      | HEAD A | HEAD B |"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 5,
          text: S("      | row 1A | row 1B |"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 6,
          text: S("      | row 2A | row 2B |"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 7,
          text: S("    And step 2"),
          indent: 4,
          line_type: LineType::StepStart,
        },
      ];
      pretty::assert_eq!(have_lines, want_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines, "file.feature".into()).unwrap();
      let want_feature = parser::Feature {
        blocks: vec![
          Block::NonExecutable(NonExecutableBlock {
            line_no: 0,
            text: vec![S("Feature: test"), S("")],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Scenario: with table"),
            line_no: 2,
            steps: vec![
              Step {
                lines: vec![
                  S("    Given step 1:"),
                  S("      | HEAD A | HEAD B |"),
                  S("      | row 1A | row 1B |"),
                  S("      | row 2A | row 2B |"),
                ],
                title: S("step 1:"),
                line_no: 3,
              },
              Step {
                lines: vec![S("    And step 2")],
                title: S("step 2"),
                line_no: 7,
              },
            ],
          }),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);
    }

    #[test]
    fn scenario_outline() {
      let give = r#"
Feature: test

  Scenario Outline:
    Given <ALPHA>
    Then <BETA>

    Examples:
      | ALPHA | BETA |
      | one   | two  |
"#;
      let bufread = BufReader::new(&give.as_bytes()[1..]);
      let have = lexer::file(bufread);
      let want = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 1,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 2,
          text: S("  Scenario Outline:"),
          indent: 2,
          line_type: LineType::BlockStart,
        },
        Line {
          number: 3,
          text: S("    Given <ALPHA>"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 4,
          text: S("    Then <BETA>"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 5,
          text: S(""),
          indent: 0,
          line_type: LineType::Other,
        },
        Line {
          number: 6,
          text: S("    Examples:"),
          indent: 4,
          line_type: LineType::Other,
        },
        Line {
          number: 7,
          text: S("      | ALPHA | BETA |"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 8,
          text: S("      | one   | two  |"),
          indent: 6,
          line_type: LineType::Other,
        },
      ];
      pretty::assert_eq!(want, have);
    }
  }
}
