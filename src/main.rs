mod cli;
mod cmd;
mod config;
mod domain;
mod find;
mod gherkin;
mod prelude;
mod sort;

use cli::Command::{Check, Format, Help};
use prelude::*;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
  match inner() {
    Ok(exit_code) => exit_code,
    Err(err) => {
      println!("{}", err);
      ExitCode::FAILURE
    }
  }
}

fn inner() -> Result<ExitCode> {
  match cli::load(env::args())? {
    Check { file } => cmd::check(file),
    Format { file } => cmd::format(file),
    Help => cmd::help(),
  }
}
