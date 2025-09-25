use crate::prelude::*;
use camino::Utf8PathBuf;
use std::process::ExitCode;

pub fn format(file: Option<Utf8PathBuf>) -> Result<ExitCode> {
  Ok(ExitCode::SUCCESS)
}
