use crate::prelude::*;

pub fn help() -> Result<usize> {
    println!("Sorts steps in Gherkin files to match the order in cucumbersortrc.");
    println!("{}", available_commands());
    Ok(0)
}

pub fn available_commands() -> &'static str {
    r#"Available commands:

check: verifies ordering of the Cucumber files
format: fixes the order of Cucumber files"#
}
