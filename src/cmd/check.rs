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
  let sorted_file = sort::file(gherkin.clone(), config, &mut issues);
  let sorted_lines = sorted_file.lines();
  println!("SORTED LINES: {:?}", sorted_lines);
  let original_lines = gherkin.lines();
  println!("ORIGINAL LINES: {:?}", sorted_lines);
  let mut exit_code = ExitCode::SUCCESS;
  original_lines.find_mismatching(&sorted_lines, &filepath, &mut issues);
  sort::sort_issues(&mut issues);
  for issue in issues {
    exit_code = ExitCode::FAILURE;
    println!("{}", issue.problem);
  }
  Ok(exit_code)
}

fn check_all(config: config::Config) -> Result<ExitCode> {
  for filepath in find::all()? {
    check_file(filepath, &config)?;
  }
  Ok(ExitCode::SUCCESS)
}
