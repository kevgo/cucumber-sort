mod lexer;
mod parser;

pub use parser::{Block, Feature, Step};

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
        use crate::gherkin::lexer::{self, Indentation, Line, LineType, TrimmedLine};
        use crate::gherkin::{Block, Step, parser};
        use big_s::S;
        use std::io::BufReader;

        #[test]
        fn simple() {
            // step 1: lex Gherkin source into tokens (in our case Lines)
            let source = r#"
Feature: test

  An example feature file.

  Background:
    Given step 1
    And step 2:
      """
      docstring line 1
      docstring line 2
      """
    When step 3

  Scenario: result
    Then step 4:
      | HEADING 1 | HEADING 2 |
      | line 1a   | line 1b   |
      | line 2a   | line 2b   |
"#;
            let have_lines = lexer::file(BufReader::new(source[1..].as_bytes()));
            let want_lines = vec![
                Line {
                    number: 0,
                    full_text: S("Feature: test"),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from("Feature: test"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 1,
                    full_text: S(""),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 2,
                    full_text: S("  An example feature file."),
                    indent: Indentation::new(2),
                    trimmed_text: TrimmedLine::from("An example feature file."),
                    line_type: LineType::Other,
                },
                Line {
                    number: 3,
                    full_text: S(""),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 4,
                    full_text: S("  Background:"),
                    indent: Indentation::new(2),
                    trimmed_text: TrimmedLine::from("Background:"),
                    line_type: LineType::BlockStart,
                },
                Line {
                    number: 5,
                    full_text: S("    Given step 1"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("Given step 1"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 6,
                    full_text: S("    And step 2:"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("And step 2:"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 7,
                    full_text: S(r#"      """"#),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from(r#"""""#),
                    line_type: LineType::Other,
                },
                Line {
                    number: 8,
                    full_text: S("      docstring line 1"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("docstring line 1"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 9,
                    full_text: S("      docstring line 2"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("docstring line 2"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 10,
                    full_text: S(r#"      """"#),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from(r#"""""#),
                    line_type: LineType::Other,
                },
                Line {
                    number: 11,
                    full_text: S("    When step 3"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("When step 3"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 12,
                    full_text: S(""),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 13,
                    full_text: S("  Scenario: result"),
                    indent: Indentation::new(2),
                    trimmed_text: TrimmedLine::from("Scenario: result"),
                    line_type: LineType::BlockStart,
                },
                Line {
                    number: 14,
                    full_text: S("    Then step 4:"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("Then step 4:"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 15,
                    full_text: S("      | HEADING 1 | HEADING 2 |"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("| HEADING 1 | HEADING 2 |"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 16,
                    full_text: S("      | line 1a   | line 1b   |"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("| line 1a   | line 1b   |"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 17,
                    full_text: S("      | line 2a   | line 2b   |"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("| line 2a   | line 2b   |"),
                    line_type: LineType::Other,
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
                        start_line: 4,
                        steps: vec![
                            Step {
                                lines: vec![S("  Background:")],
                                title: S("Background:"),
                            },
                            Step {
                                lines: vec![S("    Given step 1")],
                                title: S("Given step 1"),
                            },
                            Step {
                                lines: vec![
                                    S("    And step 2:"),
                                    S("      \"\"\""),
                                    S("      docstring line 1"),
                                    S("      docstring line 2"),
                                    S("      \"\"\""),
                                ],
                                title: S("And step 2:"),
                            },
                            Step {
                                lines: vec![S("    When step 3"), S("")],
                                title: S("When step 3"),
                            },
                        ],
                    },
                    Block {
                        start_line: 13,
                        steps: vec![Step {
                            lines: vec![S("  Scenario: result")],
                            title: S("Scenario: result"),
                        }],
                    },
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
            let bufread = BufReader::new(give[1..].as_bytes());
            let have = lexer::file(bufread);
            let want = vec![
                Line {
                    number: 0,
                    full_text: S("Feature: test"),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from("Feature: test"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 1,
                    full_text: S(""),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 2,
                    full_text: S("  Scenario Outline:"),
                    indent: Indentation::new(2),
                    trimmed_text: TrimmedLine::from("Scenario Outline:"),
                    line_type: LineType::BlockStart,
                },
                Line {
                    number: 3,
                    full_text: S("    Given <ALPHA>"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("Given <ALPHA>"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 4,
                    full_text: S("    Then <BETA>"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("Then <BETA>"),
                    line_type: LineType::StepStart,
                },
                Line {
                    number: 5,
                    full_text: S(""),
                    indent: Indentation::new(0),
                    trimmed_text: TrimmedLine::from(""),
                    line_type: LineType::Other,
                },
                Line {
                    number: 6,
                    full_text: S("    Examples:"),
                    indent: Indentation::new(4),
                    trimmed_text: TrimmedLine::from("Examples:"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 7,
                    full_text: S("      | ALPHA | BETA |"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("| ALPHA | BETA |"),
                    line_type: LineType::Other,
                },
                Line {
                    number: 8,
                    full_text: S("      | one   | two  |"),
                    indent: Indentation::new(6),
                    trimmed_text: TrimmedLine::from("| one   | two  |"),
                    line_type: LineType::Other,
                },
            ];
            pretty::assert_eq!(want, have);
        }
    }
}
