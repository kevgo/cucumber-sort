mod lexer;
mod parser;

use crate::domain;
use crate::prelude::*;
use std::io::BufRead;

pub fn file(text: impl BufRead) -> Result<domain::File> {
    Ok(domain::File {
        initial_lines: vec![],
        blocks: vec![],
    })
}

/*
parsing Gherkin happens in several steps:

Lexing: parse the text into lines with type (is block start, is step start, other line) and indentation

Parsing: group lines into steps
- any non-step-def following a step def and having a bigger indentation belongs to the step
- the only difference is if a matching non-step-def line contains `"""` --> capture all subsequent lines until you see a similar line
- a step is finished if we encounter an empty line

*/
