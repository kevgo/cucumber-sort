mod cli;
mod cmd;
mod config;
mod errors;
mod file_finder;
mod gherkin;

use crate::errors::Result;
use ansi_term::Color::Red;
use cli::Command::{Check, Format, Init};
use file_finder::FileFinder;
use std::process::ExitCode;

fn main() -> ExitCode {
  match inner() {
    Ok(exit_code) => exit_code,
    Err(err) => {
      let (message, details) = err.messages();
      println!("{}", Red.paint(message));
      if let Some(details) = details {
        println!("\n{}", details);
      }
      ExitCode::FAILURE
    }
  }
}

fn inner() -> Result<ExitCode> {
  match cli::parse() {
    Check { file } => cmd::check(file),
    Format { file } => cmd::format(file),
    Init => cmd::init(),
  }
}
