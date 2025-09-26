mod lexer;
mod parser;

use crate::prelude::*;
use camino::Utf8Path;
pub use parser::{Block, Feature, Step};
use std::fs;
use std::io::{BufRead, BufReader};

pub fn load(filepath: &Utf8Path) -> Result<parser::Feature> {
  let file_content = fs::File::open(filepath).map_err(|e| UserError::CannotReadFile {
    file: filepath.to_path_buf(),
    reason: e.to_string(),
  })?;
  file(BufReader::new(file_content))
}

/// parses the given file content into Gherkin
pub fn file(text: impl BufRead) -> Result<parser::Feature> {
  // step 1: lex the file content into token (lines)
  let lines = lexer::file(text);
  // step 2: parse the tokens (lines) into Gherkin data structures
  parser::file(lines)
}

#[cfg(test)]
mod tests {

  mod lex_and_parse {
    use crate::gherkin::lexer::{self, Line, LineType};
    use crate::gherkin::parser::Lines;
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
          line_type: LineType::Other,
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
          line_type: LineType::Other,
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
          line_type: LineType::Other,
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
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Feature {
        blocks: vec![
          Block::Text(vec![
            S("Feature: test"),
            S(""),
            S("  An example feature file."),
            S(""),
          ]),
          Block::Steps(vec![
            Step {
              title: S("step 1"),
              lines: vec![S("    Given step 1")],
              indent: 4,
            },
            Step {
              title: S("step 2"),
              lines: vec![S("    And step 2")],
              indent: 4,
            },
            Step {
              title: S("step 3"),
              lines: vec![S("    When step 3")],
              indent: 4,
            },
          ]),
          Block::Text(vec![S("")]),
          Block::Steps(vec![
            Step {
              title: S("step 4"),
              lines: vec![S("    Then step 4")],
              indent: 4,
            },
            Step {
              title: S("step 5"),
              lines: vec![S("    And step 5")],
              indent: 4,
            },
          ]),
          Block::Text(vec![S("")]),
          Block::Steps(vec![
            Step {
              title: S("step 6"),
              lines: vec![S("    When step 6")],
              indent: 4,
            },
            Step {
              title: S("step 7"),
              lines: vec![S("    Then step 7")],
              indent: 4,
            },
          ]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        (0, S("Feature: test")),
        (1, S("")),
        (2, S("  An example feature file.")),
        (3, S("")),
        (4, S("  Background:")),
        (5, S("    Given step 1")),
        (6, S("    And step 2")),
        (7, S("    When step 3")),
        (8, S("")),
        (9, S("  Scenario: result")),
        (10, S("    Then step 4")),
        (11, S("    And step 5")),
        (12, S("")),
        (13, S("  Scenario: undo")),
        (14, S("    When step 6")),
        (15, S("    Then step 7")),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source, have_text);
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
          line_type: LineType::Other,
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
          line_type: LineType::CommentStartStop,
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
          line_type: LineType::CommentStartStop,
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
          line_type: LineType::Other,
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
      let have_lines = lexer::file(bufread);
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
          text: S("  Scenario Outline:"),
          indent: 2,
          line_type: LineType::Other,
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
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines, "file.feature".into()).unwrap();
      let want_feature = parser::Feature {
        blocks: vec![
          Block::NonExecutable(NonExecutableBlock {
            line_no: 0,
            text: vec![S("Feature: test"), S("")],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Scenario Outline:"),
            line_no: 2,
            steps: vec![
              Step {
                title: S("<ALPHA>"),
                line_no: 3,
                lines: vec![S("    Given <ALPHA>")],
              },
              Step {
                title: S("<BETA>"),
                line_no: 4,
                lines: vec![
                  S("    Then <BETA>"),
                  S(""),
                  S("    Examples:"),
                  S("      | ALPHA | BETA |"),
                  S("      | one   | two  |"),
                ],
              },
            ],
          }),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);
    }

    #[test]
    fn cucumber_in_docstring() {
      let give = r#"
Feature: test

  Scenario: gherkin in docstring
    Given file "foo":
      """
      Scenario: embedded
        Given step 1
      """
    When step 2
"#;
      let bufread = BufReader::new(&give.as_bytes()[1..]);
      let have_lines = lexer::file(bufread);
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
          text: S("  Scenario: gherkin in docstring"),
          indent: 2,
          line_type: LineType::Other,
        },
        Line {
          number: 3,
          text: S("    Given file \"foo\":"),
          indent: 4,
          line_type: LineType::StepStart,
        },
        Line {
          number: 4,
          text: S(r#"      """"#),
          indent: 6,
          line_type: LineType::CommentStartStop,
        },
        Line {
          number: 5,
          text: S("      Scenario: embedded"),
          indent: 6,
          line_type: LineType::Other,
        },
        Line {
          number: 6,
          text: S("        Given step 1"),
          indent: 8,
          line_type: LineType::Other,
        },
        Line {
          number: 7,
          text: S("      \"\"\""),
          indent: 6,
          line_type: LineType::CommentStartStop,
        },
        Line {
          number: 8,
          text: S("    When step 2"),
          indent: 4,
          line_type: LineType::StepStart,
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines, "file.feature".into()).unwrap();
      let want_feature = parser::Feature {
        blocks: vec![
          Block::NonExecutable(NonExecutableBlock {
            line_no: 0,
            text: vec![S("Feature: test"), S("")],
          }),
          Block::Executable(ExecutableBlock {
            title: S("  Scenario: gherkin in docstring"),
            line_no: 2,
            steps: vec![
              Step {
                title: S("file \"foo\":"),
                line_no: 3,
                lines: vec![
                  S("    Given file \"foo\":"),
                  S("      \"\"\""),
                  S("      Scenario: embedded"),
                  S("        Given step 1"),
                  S("      \"\"\""),
                ],
              },
              Step {
                title: S("step 2"),
                line_no: 8,
                lines: vec![S("    When step 2")],
              },
            ],
          }),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);
    }
  }
}
