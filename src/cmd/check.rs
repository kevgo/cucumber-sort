use crate::prelude::*;
use crate::sort::{self, Issue};
use crate::{config, find, gherkin};
use ansi_term::Color::{Cyan, Red};
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
  let original_lines = gherkin.lines();
  let mut exit_code = ExitCode::SUCCESS;
  for ((original_line, original_text), (_, sorted_text)) in
    original_lines.into_iter().zip(sorted_lines.into_iter())
  {
    if original_text != *sorted_text {
      issues.push(Issue {
        line: original_line,
        problem: format!(
          "{filepath}:{original_line}  expected {} but found {}",
          Cyan.paint(sorted_text.trim()),
          Red.paint(original_text.trim())
        ),
      });
    }
  }
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
