use regex::Regex;

use crate::prelude::*;
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
  Ok(Config {
    steps: BufReader::new(file)
      .lines()
      .map(|e| e.unwrap())
      .filter(|line| !line.is_empty())
      .map(|line| Regex::new(&line).unwrap())
      .collect(),
  })
}
