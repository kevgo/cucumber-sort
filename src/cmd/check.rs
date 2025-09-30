use crate::errors::{Result, sort_issues};
use crate::gherkin::Sorter;
use crate::{config, gherkin};
use camino::Utf8PathBuf;
use std::process::ExitCode;

/// verifies whether the given or all files contain sorted steps
pub fn check(filepath: Option<Utf8PathBuf>) -> Result<ExitCode> {
  let config = config::load()?;
  match filepath {
    Some(filepath) => file(filepath, &config.sorter),
    None => all(&config),
  }
}

/// checks all files in the current folder
fn all(config: &config::Config) -> Result<ExitCode> {
  for filepath in config.globber.search_folder(".")? {
    let exit_code = file(filepath, &config.sorter)?;
    if exit_code != ExitCode::SUCCESS {
      return Ok(exit_code);
    }
  }
  Ok(ExitCode::SUCCESS)
}

/// checks the file with the given path
fn file(filepath: Utf8PathBuf, sorter: &Sorter) -> Result<ExitCode> {
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, mut issues) = sorter.sort_file(gherkin.clone(), &filepath);
  let sorted_lines = sorted_file.lines();
  let original_lines = gherkin.lines();
  issues.extend(original_lines.find_mismatching(&sorted_lines, &filepath));
  sort_issues(&mut issues);
  for issue in &issues {
    println!("{}", issue.problem);
  }
  if issues.is_empty() {
    Ok(ExitCode::SUCCESS)
  } else {
    Ok(ExitCode::FAILURE)
  }
}
