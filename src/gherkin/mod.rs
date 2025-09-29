mod lexer;
mod parser;

use crate::prelude::*;
use camino::Utf8Path;
pub use lexer::Keyword;
pub use parser::{Block, Document, Step};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filepath: &Utf8Path) -> Result<parser::Document> {
  let file_content = File::open(filepath).map_err(|e| UserError::FileRead {
    file: filepath.to_path_buf(),
    reason: e.to_string(),
  })?;
  file(BufReader::new(file_content))
}

/// parses the given file content into Gherkin
pub fn file(text: impl BufRead) -> Result<parser::Document> {
  // step 1: lex the file content into token (lines)
  let lines = lexer::file(text)?;
  // step 2: parse the tokens (lines) into Gherkin data structures
  parser::file(lines)
}

#[cfg(test)]
mod tests {

  mod lex_and_parse {
    use crate::gherkin::lexer::{self, Keyword, Line, LineType};
    use crate::gherkin::parser::Lines;
    use crate::gherkin::{Block, Step, parser};
    use big_s::S;
    use std::io::BufReader;

    #[test]
    fn multiple_scenarios() {
      // step 1: lex Gherkin source into Lines
      let source = r#"
Feature: test

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
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..])).unwrap();
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 1,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 2,
          text: S("  Background:"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 3,
          text: S("    Given step 1"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Given,
          },
        },
        Line {
          number: 4,
          text: S("    And step 2"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::And,
          },
        },
        Line {
          number: 5,
          text: S("    When step 3"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::When,
          },
        },
        Line {
          number: 6,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 7,
          text: S("  Scenario: result"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 8,
          text: S("    Then step 4"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Then,
          },
        },
        Line {
          number: 9,
          text: S("    And step 5"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::And,
          },
        },
        Line {
          number: 10,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 11,
          text: S("  Scenario: undo"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 12,
          text: S("    When step 6"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::When,
          },
        },
        Line {
          number: 13,
          text: S("    Then step 7"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Then,
          },
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Document {
        blocks: vec![
          Block::Static(vec![S("Feature: test"), S(""), S("  Background:")]),
          Block::Sortable(vec![
            Step {
              title: S("step 1"),
              keyword: Keyword::Given,
              additional_lines: vec![S("    Given step 1")],
              indent: S("    "),
              line_no: 3,
            },
            Step {
              title: S("step 2"),
              keyword: Keyword::And,
              additional_lines: vec![S("    And step 2")],
              indent: S("    "),
              line_no: 4,
            },
            Step {
              title: S("step 3"),
              keyword: Keyword::When,
              additional_lines: vec![S("    When step 3")],
              indent: S("    "),
              line_no: 5,
            },
          ]),
          Block::Static(vec![S(""), S("  Scenario: result")]),
          Block::Sortable(vec![
            Step {
              title: S("step 4"),
              keyword: Keyword::Then,
              additional_lines: vec![S("    Then step 4")],
              indent: S("    "),
              line_no: 8,
            },
            Step {
              title: S("step 5"),
              keyword: Keyword::And,
              additional_lines: vec![S("    And step 5")],
              indent: S("    "),
              line_no: 9,
            },
          ]),
          Block::Static(vec![S(""), S("  Scenario: undo")]),
          Block::Sortable(vec![
            Step {
              title: S("step 6"),
              keyword: Keyword::When,
              additional_lines: vec![S("    When step 6")],
              indent: S("    "),
              line_no: 12,
            },
            Step {
              title: S("step 7"),
              keyword: Keyword::Then,
              additional_lines: vec![S("    Then step 7")],
              indent: S("    "),
              line_no: 13,
            },
          ]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        S("Feature: test"),
        S(""),
        S("  Background:"),
        S("    Given step 1"),
        S("    And step 2"),
        S("    When step 3"),
        S(""),
        S("  Scenario: result"),
        S("    Then step 4"),
        S("    And step 5"),
        S(""),
        S("  Scenario: undo"),
        S("    When step 6"),
        S("    Then step 7"),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source[1..], have_text);
    }

    #[test]
    fn comments() {
      // step 1: lex Gherkin source into Lines
      let source = r#"
Feature: test

  An example feature file

  Scenario:
    Given step 1:
    # And step 2
    And step 3
"#;
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..])).unwrap();
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 1,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 2,
          text: S("  An example feature file"),
          indent: S("    "),
          line_type: LineType::Text,
        },
        Line {
          number: 3,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 4,
          text: S("  Scenario:"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 5,
          text: S("    Given step 1:"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Given,
          },
        },
        Line {
          number: 6,
          text: S("    # And step 2"),
          indent: S("    "),
          line_type: LineType::Text,
        },
        Line {
          number: 7,
          text: S("    And step 3"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::And,
          },
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Document {
        blocks: vec![
          Block::Static(vec![
            S("Feature: test"),
            S(""),
            S("  An example feature file"),
            S(""),
            S("  Scenario:"),
          ]),
          Block::Sortable(vec![Step {
            title: S("step 1:"),
            keyword: Keyword::Given,
            additional_lines: vec![S("    Given step 1:")],
            indent: S("    "),
            line_no: 5,
          }]),
          Block::Static(vec![S("    # And step 2")]),
          Block::Sortable(vec![Step {
            title: S("step 3"),
            keyword: Keyword::And,
            additional_lines: vec![S("    And step 3")],
            indent: S("    "),
            line_no: 7,
          }]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        S("Feature: test"),
        S(""),
        S("  An example feature file"),
        S(""),
        S("  Scenario:"),
        S("    Given step 1:"),
        S("    # And step 2"),
        S("    And step 3"),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source[1..], have_text);
    }

    #[test]
    fn docstrings() {
      // step 1: lex Gherkin source into Lines
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
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..])).unwrap();
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 1,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 2,
          text: S("  Scenario: with docstring"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 3,
          text: S("    Given step 1:"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Given,
          },
        },
        Line {
          number: 4,
          text: S(r#"      """"#),
          indent: S("      "),
          line_type: LineType::DocStringStartStop,
        },
        Line {
          number: 5,
          text: S("      docstring line 1"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 6,
          text: S("      docstring line 2"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 7,
          text: S(r#"      """"#),
          indent: S("      "),
          line_type: LineType::DocStringStartStop,
        },
        Line {
          number: 8,
          text: S("    And step 2"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::And,
          },
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Document {
        blocks: vec![
          Block::Static(vec![
            S("Feature: test"),
            S(""),
            S("  Scenario: with docstring"),
          ]),
          Block::Sortable(vec![
            Step {
              title: S("step 1:"),
              keyword: Keyword::Given,
              additional_lines: vec![
                S("    Given step 1:"),
                S("      \"\"\""),
                S("      docstring line 1"),
                S("      docstring line 2"),
                S("      \"\"\""),
              ],
              indent: S("    "),
              line_no: 3,
            },
            Step {
              title: S("step 2"),
              keyword: Keyword::And,
              additional_lines: vec![S("    And step 2")],
              indent: S("    "),
              line_no: 8,
            },
          ]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        S("Feature: test"),
        S(""),
        S("  Scenario: with docstring"),
        S("    Given step 1:"),
        S("      \"\"\""),
        S("      docstring line 1"),
        S("      docstring line 2"),
        S("      \"\"\""),
        S("    And step 2"),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source[1..], have_text);
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
      let have_lines = lexer::file(BufReader::new(&source.as_bytes()[1..])).unwrap();
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 1,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 2,
          text: S("  Scenario: with table"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 3,
          text: S("    Given step 1:"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Given,
          },
        },
        Line {
          number: 4,
          text: S("      | HEAD A | HEAD B |"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 5,
          text: S("      | row 1A | row 1B |"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 6,
          text: S("      | row 2A | row 2B |"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 7,
          text: S("    And step 2"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::And,
          },
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Document {
        blocks: vec![
          Block::Static(vec![S("Feature: test"), S(""), S("  Scenario: with table")]),
          Block::Sortable(vec![Step {
            title: S("step 1:"),
            keyword: Keyword::Given,
            additional_lines: vec![S("    Given step 1:")],
            indent: S("    "),
            line_no: 3,
          }]),
          Block::Static(vec![
            S("      | HEAD A | HEAD B |"),
            S("      | row 1A | row 1B |"),
            S("      | row 2A | row 2B |"),
          ]),
          Block::Sortable(vec![Step {
            title: S("step 2"),
            keyword: Keyword::And,
            additional_lines: vec![S("    And step 2")],
            indent: S("    "),
            line_no: 7,
          }]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        S("Feature: test"),
        S(""),
        S("  Scenario: with table"),
        S("    Given step 1:"),
        S("      | HEAD A | HEAD B |"),
        S("      | row 1A | row 1B |"),
        S("      | row 2A | row 2B |"),
        S("    And step 2"),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source[1..], have_text);
    }

    #[test]
    fn scenario_outline() {
      let source = r#"
Feature: test

  Scenario Outline:
    Given <ALPHA>
    Then <BETA>

    Examples:
      | ALPHA | BETA |
      | one   | two  |
"#;
      let bufread = BufReader::new(&source.as_bytes()[1..]);
      let have_lines = lexer::file(bufread).unwrap();
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 1,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 2,
          text: S("  Scenario Outline:"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 3,
          text: S("    Given <ALPHA>"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Given,
          },
        },
        Line {
          number: 4,
          text: S("    Then <BETA>"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Then,
          },
        },
        Line {
          number: 5,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 6,
          text: S("    Examples:"),
          indent: S("    "),
          line_type: LineType::Text,
        },
        Line {
          number: 7,
          text: S("      | ALPHA | BETA |"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 8,
          text: S("      | one   | two  |"),
          indent: S("      "),
          line_type: LineType::Text,
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Document {
        blocks: vec![
          Block::Static(vec![S("Feature: test"), S(""), S("  Scenario Outline:")]),
          Block::Sortable(vec![
            Step {
              title: S("<ALPHA>"),
              keyword: Keyword::Given,
              additional_lines: vec![S("    Given <ALPHA>")],
              indent: S("    "),
              line_no: 3,
            },
            Step {
              title: S("<BETA>"),
              keyword: Keyword::Then,
              additional_lines: vec![S("    Then <BETA>")],
              indent: S("    "),
              line_no: 4,
            },
          ]),
          Block::Static(vec![
            S(""),
            S("    Examples:"),
            S("      | ALPHA | BETA |"),
            S("      | one   | two  |"),
          ]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        S("Feature: test"),
        S(""),
        S("  Scenario Outline:"),
        S("    Given <ALPHA>"),
        S("    Then <BETA>"),
        S(""),
        S("    Examples:"),
        S("      | ALPHA | BETA |"),
        S("      | one   | two  |"),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source[1..], have_text);
    }

    #[test]
    fn cucumber_in_docstring() {
      let source = r#"
Feature: test

  Scenario: gherkin in docstring
    Given file "foo":
      """
      Scenario: embedded
        Given step 1
      """
    When step 2
"#;
      let bufread = BufReader::new(&source.as_bytes()[1..]);
      let have_lines = lexer::file(bufread).unwrap();
      let want_lines = vec![
        Line {
          number: 0,
          text: S("Feature: test"),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 1,
          text: S(""),
          indent: S(""),
          line_type: LineType::Text,
        },
        Line {
          number: 2,
          text: S("  Scenario: gherkin in docstring"),
          indent: S("  "),
          line_type: LineType::Text,
        },
        Line {
          number: 3,
          text: S("    Given file \"foo\":"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::Given,
          },
        },
        Line {
          number: 4,
          text: S(r#"      """"#),
          indent: S("      "),
          line_type: LineType::DocStringStartStop,
        },
        Line {
          number: 5,
          text: S("      Scenario: embedded"),
          indent: S("      "),
          line_type: LineType::Text,
        },
        Line {
          number: 6,
          text: S("        Given step 1"),
          indent: S("        "),
          line_type: LineType::Text,
        },
        Line {
          number: 7,
          text: S("      \"\"\""),
          indent: S("      "),
          line_type: LineType::DocStringStartStop,
        },
        Line {
          number: 8,
          text: S("    When step 2"),
          indent: S("    "),
          line_type: LineType::StepStart {
            keyword: Keyword::When,
          },
        },
      ];
      pretty::assert_eq!(want_lines, have_lines);

      // step 2: parse the Lines into blocks
      let have_feature = parser::file(have_lines).unwrap();
      let want_feature = parser::Document {
        blocks: vec![
          Block::Static(vec![
            S("Feature: test"),
            S(""),
            S("  Scenario: gherkin in docstring"),
          ]),
          Block::Sortable(vec![
            Step {
              title: S("file \"foo\":"),
              keyword: Keyword::Given,
              additional_lines: vec![
                S("    Given file \"foo\":"),
                S("      \"\"\""),
                S("      Scenario: embedded"),
                S("        Given step 1"),
                S("      \"\"\""),
              ],
              indent: S("    "),
              line_no: 3,
            },
            Step {
              title: S("step 2"),
              keyword: Keyword::When,
              additional_lines: vec![S("    When step 2")],
              indent: S("    "),
              line_no: 8,
            },
          ]),
        ],
      };
      pretty::assert_eq!(want_feature, have_feature);

      // step 3: serialize the block back into lines
      let have_lines = have_feature.lines();
      let want_lines = Lines::from(vec![
        S("Feature: test"),
        S(""),
        S("  Scenario: gherkin in docstring"),
        S("    Given file \"foo\":"),
        S("      \"\"\""),
        S("      Scenario: embedded"),
        S("        Given step 1"),
        S("      \"\"\""),
        S("    When step 2"),
      ]);
      pretty::assert_eq!(want_lines, have_lines);

      // step 4: serialize back into the original string
      let have_text = have_lines.to_string();
      pretty::assert_eq!(source[1..], have_text);
    }
  }
}
