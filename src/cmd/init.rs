use crate::config;
use crate::errors::Result;
use std::process::ExitCode;

pub fn init() -> Result<ExitCode> {
  config::create()?;
  println!("config files created");
  Ok(ExitCode::SUCCESS)
}
