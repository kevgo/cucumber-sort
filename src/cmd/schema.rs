use crate::config;
use crate::errors::AppResult;
use std::process::ExitCode;

pub fn schema() -> AppResult<ExitCode> {
  let schema = config::generate_schema();
  println!("{}", schema);
  Ok(ExitCode::SUCCESS)
}
