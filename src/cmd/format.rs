use crate::errors::{Result, UserError};
use crate::gherkin::Sorter;
use crate::{config, gherkin};
use camino::Utf8PathBuf;
use std::fs;
use std::process::ExitCode;

/// updates the given or all files to contain sorted steps
pub fn format(filepath: Option<Utf8PathBuf>, record: bool) -> Result<ExitCode> {
  let mut config = config::load()?;
  match filepath {
    Some(filepath) => file(filepath, &mut config.sorter),
    None => all(config),
  }
}

/// updates all files in the current folder to contain sorted steps
fn all(mut config: config::Config) -> Result<ExitCode> {
  for filepath in config.finder.search_folder(".")? {
    let exit_code = file(filepath, &mut config.sorter)?;
    if exit_code != ExitCode::SUCCESS {
      return Ok(exit_code);
    }
  }
  for unused in config.sorter.unused_regexes() {
    println!("{}", unused);
  }
  Ok(ExitCode::SUCCESS)
}

/// updates the given file to contain sorted steps
fn file(filepath: Utf8PathBuf, sorter: &mut Sorter) -> Result<ExitCode> {
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, issues) = sorter.sort_file(gherkin.clone(), &filepath);
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
