use crate::errors::{Finding, Result};
use crate::gherkin::Sorter;
use crate::{config, gherkin};
use camino::Utf8PathBuf;
use std::process::ExitCode;

/// verifies whether the given or all files contain sorted steps
pub fn check(filepath: Option<Utf8PathBuf>, record: bool, fail_fast: bool) -> Result<ExitCode> {
  let mut config = config::load()?;
  let mut findings = match filepath {
    Some(filepath) => file(filepath, &mut config.sorter),
    None => all(&mut config, fail_fast),
  }?;
  findings.sort();
  for finding in &findings {
    println!("{}", finding);
  }
  if record {
    config.sorter.store_missing(&findings)?;
  }
  if findings.is_empty() {
    Ok(ExitCode::SUCCESS)
  } else {
    Ok(ExitCode::FAILURE)
  }
}

/// checks all files in the current folder
fn all(config: &mut config::Config, fail_fast: bool) -> Result<Vec<Finding>> {
  let mut result = vec![];
  for filepath in config.finder.search_folder(".")? {
    let findings = file(filepath, &mut config.sorter)?;
    let found_problems = !findings.is_empty();
    result.extend(findings);
    if fail_fast && found_problems {
      break;
    }
  }
  result.extend(config.sorter.unused_regexes());
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
