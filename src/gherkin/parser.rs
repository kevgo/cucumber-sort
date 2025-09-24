use crate::gherkin::lexer;
use crate::{domain, prelude::*};

pub fn file(lines: Vec<lexer::Line>) -> Result<domain::File> {
    Ok(domain::File {
        initial_lines: vec![],
        blocks: vec![],
    })
}
