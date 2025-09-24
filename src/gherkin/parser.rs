use crate::gherkin::lexer;
use crate::{domain, prelude::*};

pub fn file(lines: Vec<lexer::Line>) -> domain::File {
    domain::File {
        initial_lines: vec![],
        blocks: vec![],
    }
}
