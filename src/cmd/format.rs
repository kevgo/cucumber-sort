use crate::errors::{AppFinding, Result, UserError};
use crate::gherkin::Sorter;
use crate::{config, gherkin};
use camino::Utf8PathBuf;
use std::fs;
use std::process::ExitCode;

/// updates the given or all files to contain sorted steps
pub fn format(filepath: Option<Utf8PathBuf>, record: bool) -> Result<ExitCode> {
  let mut config = config::load()?;
  let mut findings = match filepath {
    Some(filepath) => file(filepath, &mut config.sorter),
    None => all(&mut config),
  }?;
  findings.sort();
  if record {
    config.sorter.record_missing(&findings)?;
  }
  if findings.is_empty() {
    Ok(ExitCode::SUCCESS)
  } else {
    Ok(ExitCode::FAILURE)
  }
}

/// updates all files in the current folder to contain sorted steps
fn all(config: &mut config::Config) -> Result<Vec<AppFinding>> {
  let mut result = vec![];
  for filepath in config.finder.search_folder(".")? {
    let findings = file(filepath, &mut config.sorter)?;
    result.extend(findings);
  }
  result.extend(config.sorter.unused_regexes());
  Ok(result)
}

/// updates the given file to contain sorted steps
fn file(filepath: Utf8PathBuf, sorter: &mut Sorter) -> Result<Vec<AppFinding>> {
  let gherkin = gherkin::load(&filepath)?;
  let (sorted_file, findings) = sorter.sort_file(gherkin.clone(), &filepath);
  let sorted_text = sorted_file.lines().to_string();
  fs::write(&filepath, sorted_text).map_err(|err| UserError::FileWrite {
    file: filepath,
    reason: err.to_string(),
  })?;
  Ok(findings)
}
