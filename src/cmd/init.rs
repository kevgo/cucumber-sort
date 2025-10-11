use crate::config;
use crate::errors::AppResult;
use std::process::ExitCode;

pub fn init() -> AppResult<ExitCode> {
  config::create()?;
  println!("config file created: {}", config::CONFIG_FILE_NAME);
  Ok(ExitCode::SUCCESS)
}
