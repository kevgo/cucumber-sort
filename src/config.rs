use crate::prelude::*;
use camino::Utf8PathBuf;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};

/// the filename of the configuration file
pub const FILE_NAME: &str = ".cucumbersortrc";

pub struct Config {
  pub steps: Vec<Regex>,
}

pub fn load() -> Result<Config> {
  let file = fs::File::open(FILE_NAME).map_err(|e| UserError::ConfigFileRead {
    reason: e.to_string(),
  })?;
  let lines = BufReader::new(file).lines();
  let mut steps = vec![];
  for (i, line) in lines.enumerate() {
    let line = line.unwrap();
    if line.is_empty() {
      continue;
    }
    match Regex::new(&line) {
      Ok(regex) => steps.push(regex),
      Err(err) => {
        return Err(UserError::ConfigFileInvalidRegex {
          file: Utf8PathBuf::from(FILE_NAME),
          line: i,
          message: err.to_string(),
        });
      }
    }
  }
  Ok(Config { steps })
}
