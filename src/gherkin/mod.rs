mod lexer;
mod parser;

#[cfg(test)]
pub use parser::Step;
pub use parser::{Block, Feature};

use std::io::BufRead;

pub fn file(text: impl BufRead) -> parser::Feature {
    let lines = lexer::file(text);
    parser::file(lines)
}

/*
parsing Gherkin happens in several steps:

Lexing: parse the text into lines with type (is block start, is step start, other line) and indentation

Parsing: group lines into steps
- any non-step-def following a step def and having a bigger indentation belongs to the step
- the only difference is if a matching non-step-def line contains `"""` --> capture all subsequent lines until you see a similar line
- a step is finished if we encounter an empty line

*/

#[cfg(test)]
mod tests {

    mod lex_and_parse {
        use crate::gherkin::lexer::{self, Line, LineType};
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
            let have_lines = lexer::file(BufReader::new(source[1..].as_bytes()));
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
            let have_feature = parser::file(have_lines);
            let want_feature = parser::Feature {
                initial_lines: vec![
                    S("Feature: test"),
                    S(""),
                    S("  An example feature file."),
                    S(""),
                ],
                blocks: vec![
                    Block {
                        title_line: S("  Background:"),
                        line_number: 4,
                        steps: vec![
                            Step {
                                lines: vec![S("    Given step 1")],
                                title: S("step 1"),
                            },
                            Step {
                                lines: vec![S("    And step 2")],
                                title: S("step 2"),
                            },
                            Step {
                                lines: vec![S("    When step 3"), S("")],
                                title: S("step 3"),
                            },
                        ],
                    },
                    Block {
                        title_line: S("  Scenario: result"),
                        line_number: 9,
                        steps: vec![
                            Step {
                                lines: vec![S("    Then step 4")],
                                title: S("step 4"),
                            },
                            Step {
                                lines: vec![S("    And step 5"), S("")],
                                title: S("step 5"),
                            },
                        ],
                    },
                    Block {
                        title_line: S("  Scenario: undo"),
                        line_number: 13,
                        steps: vec![
                            Step {
                                lines: vec![S("    When step 6")],
                                title: S("step 6"),
                            },
                            Step {
                                lines: vec![S("    Then step 7")],
                                title: S("step 7"),
                            },
                        ],
                    },
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
            let have_lines = lexer::file(BufReader::new(source[1..].as_bytes()));
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
            let have_feature = parser::file(have_lines);
            let want_feature = parser::Feature {
                initial_lines: vec![S("Feature: test"), S("")],
                blocks: vec![Block {
                    title_line: S("  Scenario: with docstring"),
                    line_number: 2,
                    steps: vec![
                        Step {
                            lines: vec![
                                S("    Given step 1:"),
                                S("      \"\"\""),
                                S("      docstring line 1"),
                                S("      docstring line 2"),
                                S("      \"\"\""),
                            ],
                            title: S("step 1:"),
                        },
                        Step {
                            lines: vec![S("    And step 2")],
                            title: S("step 2"),
                        },
                    ],
                }],
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
            let have_lines = lexer::file(BufReader::new(source[1..].as_bytes()));
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
            let have_feature = parser::file(have_lines);
            let want_feature = parser::Feature {
                initial_lines: vec![S("Feature: test"), S("")],
                blocks: vec![Block {
                    title_line: S("  Scenario: with table"),
                    line_number: 2,
                    steps: vec![
                        Step {
                            lines: vec![
                                S("    Given step 1:"),
                                S("      | HEAD A | HEAD B |"),
                                S("      | row 1A | row 1B |"),
                                S("      | row 2A | row 2B |"),
                            ],
                            title: S("step 1:"),
                        },
                        Step {
                            lines: vec![S("    And step 2")],
                            title: S("step 2"),
                        },
                    ],
                }],
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
            let bufread = BufReader::new(give[1..].as_bytes());
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
