use crate::prelude::*;
use crate::{config, find_files, gherkin, sort};
use camino::Utf8PathBuf;
use std::fs;
use std::process::ExitCode;

/// updates the given or all files to contain sorted steps
pub fn format(filepath: Option<Utf8PathBuf>) -> Result<ExitCode> {
  let config = config::load()?;
  match filepath {
    Some(filepath) => file(filepath, &config),
    None => all(&config),
  }
}

/// updates all files in the current folder to contain sorted steps
fn all(config: &config::Config) -> Result<ExitCode> {
  for filepath in find_files::all()? {
    let exit_code = file(filepath, config)?;
    if exit_code != ExitCode::SUCCESS {
      return Ok(exit_code);
    }
  }
  Ok(ExitCode::SUCCESS)
}

/// updates the given file to contain sorted steps
fn file(filepath: Utf8PathBuf, config: &config::Config) -> Result<ExitCode> {
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, issues) = sort::file(gherkin.clone(), config, &filepath);
  let sorted_lines = sorted_file.lines();
  let sorted_text = sorted_lines.to_string();
  for issue in &issues {
    println!("{}", issue.problem);
  }
  if !issues.is_empty() {
    return Ok(ExitCode::FAILURE);
  }
  fs::write(&filepath, sorted_text).map_err(|err| UserError::FileWrite {
    file: filepath,
    reason: err.to_string(),
  })?;
  Ok(ExitCode::SUCCESS)
}
