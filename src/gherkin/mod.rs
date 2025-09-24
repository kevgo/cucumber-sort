mod lexer;
mod parser;

pub use parser::{Block, File, Step};

use std::io::BufRead;

pub fn file(text: impl BufRead) -> parser::File {
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
