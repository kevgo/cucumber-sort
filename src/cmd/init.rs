use crate::config;
use crate::errors::Result;
use std::process::ExitCode;

pub fn init() -> Result<ExitCode> {
  config::create()?;
  println!("config file created: {}", config::CONFIG_FILE_NAME);
  Ok(ExitCode::SUCCESS)
}
