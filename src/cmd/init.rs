use std::process::ExitCode;

use crate::config;
use crate::errors::Result;

pub fn init() -> Result<ExitCode> {
  config::create()?;
  println!("config files created");
  Ok(ExitCode::SUCCESS)
}
