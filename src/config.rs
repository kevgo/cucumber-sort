use crate::prelude::*;
use std::fs;
use std::io::{BufRead, BufReader};

/// the filename of the configuration file
pub const FILE_NAME: &str = ".cucumbersortrc";

pub struct Config {
  pub steps: Vec<String>,
}

pub fn load() -> Result<Config> {
  let file = fs::File::open(FILE_NAME).map_err(|e| UserError::CannotReadConfigFile {
    reason: e.to_string(),
  })?;
  let reader = BufReader::new(file);
  Ok(Config {
    steps: reader
      .lines()
      .map(|e| e.unwrap())
      .filter(|line| !line.is_empty())
      .collect(),
  })
}
