mod cli;
mod cmd;
mod config;
mod domain;
mod find;
mod gherkin;
mod prelude;
mod sort;

use ansi_term::Color::Red;
use cli::Command::{Check, Format, Help};
use prelude::*;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
  match inner() {
    Ok(exit_code) => exit_code,
    Err(err) => {
      let messages = err.messages();
      println!("{}", Red.paint(messages.0));
      if let Some(details) = messages.1 {
        println!("\n{}", details);
      }
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
