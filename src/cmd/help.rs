use crate::prelude::*;
use std::process::ExitCode;

pub fn help() -> Result<ExitCode> {
  println!("Sorts steps in Gherkin files to match the order in cucumbersortrc.\n");
  println!("{}", available_commands());
  Ok(ExitCode::SUCCESS)
}

pub fn available_commands() -> &'static str {
  r#"Available commands:

check: verifies ordering of the Cucumber files
format: fixes the order of Cucumber files"#
}
