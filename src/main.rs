use ansi_term::Color::Red;
use cli::Command::{Check, Format, Init};
use cucumber_sort::errors::AppResult;
use cucumber_sort::{cli, cmd};
use std::process::ExitCode;

fn main() -> ExitCode {
  match inner() {
    Ok(exit_code) => exit_code,
    Err(err) => {
      let (message, details) = err.messages();
      eprintln!("{}", Red.paint(message));
      if let Some(details) = details {
        eprintln!("\n{}", details);
      }
      ExitCode::FAILURE
    }
  }
}

fn inner() -> AppResult<ExitCode> {
  match cli::parse() {
    Check { flags, file } => cmd::check(flags, file),
    Format { flags, file } => cmd::format(flags, file),
    Init => cmd::init(),
  }
}
