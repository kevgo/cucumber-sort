use crate::prelude::*;
use crate::sort::{self, Issue};
use crate::{config, find, gherkin};
use camino::Utf8PathBuf;
use std::process::ExitCode;

pub fn check(file: Option<Utf8PathBuf>) -> Result<ExitCode> {
  let config = config::load()?;
  match file {
    Some(filepath) => check_file(filepath, &config),
    None => check_all(config),
  }
}

fn check_file(filepath: Utf8PathBuf, config: &config::Config) -> Result<ExitCode> {
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

fn check_all(config: config::Config) -> Result<ExitCode> {
  for filepath in find::all()? {
    let exit_code = check_file(filepath, &config)?;
    if exit_code == ExitCode::FAILURE {
      return Ok(exit_code);
    }
  }
  Ok(ExitCode::SUCCESS)
}
