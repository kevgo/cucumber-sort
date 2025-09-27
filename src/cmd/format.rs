use crate::prelude::*;
use crate::sort::{self, Issue};
use crate::{config, find, gherkin};
use camino::Utf8PathBuf;
use std::fs;
use std::process::ExitCode;

pub fn format(file: Option<Utf8PathBuf>) -> Result<ExitCode> {
  let config = config::load()?;
  match file {
    Some(filepath) => format_file(filepath, &config),
    None => format_all(config),
  }
}

fn format_file(filepath: Utf8PathBuf, config: &config::Config) -> Result<ExitCode> {
  let mut issues = Vec::<Issue>::new();
  let gherkin = gherkin::load(&filepath)?;
  let sorted_file = sort::file(gherkin.clone(), config, &filepath, &mut issues);
  let sorted_lines = sorted_file.lines();
  let sorted_text = sorted_lines.to_string();
  fs::write(&filepath, sorted_text).map_err(|err| UserError::CannotWriteFile {
    file: filepath,
    reason: err.to_string(),
  })?;
  Ok(ExitCode::SUCCESS)
}

fn format_all(config: config::Config) -> Result<ExitCode> {
  for filepath in find::all()? {
    format_file(filepath, &config)?;
  }
  Ok(ExitCode::SUCCESS)
}
