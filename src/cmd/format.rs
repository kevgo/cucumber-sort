use crate::cli::Flags;
use crate::config::Config;
use crate::errors::{Finding, AppResult, UserError};
use crate::file_finder::FileFinder;
use crate::gherkin::{self, Sorter, sorter};
use camino::Utf8PathBuf;
use std::fs;
use std::process::ExitCode;

/// updates the given or all files to contain sorted steps
pub fn format(flags: Flags, filepath: Option<Utf8PathBuf>) -> AppResult<ExitCode> {
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

/// updates all files in the current folder to contain sorted steps
fn all(config: &Config, sorter: &mut Sorter) -> AppResult<Vec<Finding>> {
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

/// updates the given file to contain sorted steps
fn file(filepath: Utf8PathBuf, sorter: &mut Sorter) -> AppResult<Vec<Finding>> {
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, findings) = sorter.sort_file(gherkin.clone(), &filepath);
  let sorted_text = sorted_file.lines().to_string();
  if findings.is_empty() {
    fs::write(&filepath, sorted_text).map_err(|err| UserError::FileWrite {
      file: filepath,
      reason: err.to_string(),
    })?;
  }
  Ok(findings)
}
