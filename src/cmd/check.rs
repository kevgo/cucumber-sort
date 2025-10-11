use crate::cli::Flags;
use crate::config::Config;
use crate::errors::{Finding, Result};
use crate::file_finder::FileFinder;
use crate::gherkin::{self, Sorter, sorter};
use camino::Utf8PathBuf;
use std::process::ExitCode;

/// verifies whether the given or all files contain sorted steps
pub fn check(flags: Flags, filepath: Option<Utf8PathBuf>) -> Result<ExitCode> {
  let config = Config::load()?.merge(flags);
  let mut sorter = Sorter::try_from(&config)?;
  let mut findings = match filepath {
    Some(filepath) => file(filepath, &mut sorter),
    None => all(&config, &mut sorter),
  }?;
  findings.sort();
  for finding in &findings {
    println!("{}", finding);
  }
  if config.record {
    sorter::store_missing(&findings)?;
  }
  if findings.is_empty() {
    Ok(ExitCode::SUCCESS)
  } else {
    Ok(ExitCode::FAILURE)
  }
}

/// checks all files in the current folder
fn all(config: &Config, sorter: &mut Sorter) -> Result<Vec<Finding>> {
  let mut result = vec![];
  let finder = FileFinder::try_from(config)?;
  for filepath in finder.search_folder(".")? {
    let findings = file(filepath, sorter)?;
    let found_problems = !findings.is_empty();
    result.extend(findings);
    if config.fail_fast && found_problems {
      break;
    }
  }
  result.extend(sorter.unused_regexes());
  Ok(result)
}

/// checks the file with the given path
fn file(filepath: Utf8PathBuf, sorter: &mut Sorter) -> Result<Vec<Finding>> {
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, mut findings) = sorter.sort_file(gherkin.clone(), &filepath);
  let sorted_lines = sorted_file.lines();
  let original_lines = gherkin.lines();
  if findings.is_empty() {
    findings.extend(original_lines.find_mismatching(&sorted_lines, &filepath));
  }
  Ok(findings)
}
