use crate::prelude::*;
use crate::sort::{self, Issue};
use crate::{config, find, gherkin};
use camino::Utf8PathBuf;
use std::process::ExitCode;

/// verifies whether the given or all files contain sorted steps
pub fn check(filepath: Option<Utf8PathBuf>) -> Result<ExitCode> {
  let config = config::load()?;
  match filepath {
    Some(filepath) => file(filepath, &config),
    None => all(&config),
  }
}

/// checks all files in the current folder
fn all(config: &config::Config) -> Result<ExitCode> {
  for filepath in find::all()? {
    let exit_code = file(filepath, config)?;
    if exit_code != ExitCode::SUCCESS {
      return Ok(exit_code);
    }
  }
  Ok(ExitCode::SUCCESS)
}

/// checks the file with the given path
fn file(filepath: Utf8PathBuf, config: &config::Config) -> Result<ExitCode> {
  let mut issues = Vec::<Issue>::new();
  let gherkin = gherkin::load(&filepath)?;
  let sorted_file = sort::file(gherkin.clone(), config, &filepath, &mut issues);
  let sorted_lines = sorted_file.lines();
  let original_lines = gherkin.lines();
  original_lines.find_mismatching(&sorted_lines, &filepath, &mut issues);
  sort::sort_issues(&mut issues);
  for issue in &issues {
    println!("{}", issue.problem);
  }
  if issues.is_empty() {
    Ok(ExitCode::SUCCESS)
  } else {
    Ok(ExitCode::FAILURE)
  }
}
