use crate::prelude::*;
use crate::{config, find, gherkin, sort};
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
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, mut issues) = sort::file(gherkin.clone(), config, &filepath);
  let sorted_lines = sorted_file.lines();
  let original_lines = gherkin.lines();
  let issues2 = original_lines.find_mismatching(&sorted_lines, &filepath);
  issues.extend(issues2);
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
