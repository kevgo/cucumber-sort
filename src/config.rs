use crate::filesystem::ignore_files::Ignorer;
use crate::prelude::*;
use camino::Utf8PathBuf;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};

/// the filename of the configuration file
pub const CONFIG_FILE_NAME: &str = ".cucumbersortrc";

pub struct Config {
  pub steps: Vec<Regex>,
  pub ignorer: Ignorer,
}

pub fn load() -> Result<Config> {
  Ok(Config {
    steps: load_config_file()?,
    ignorer: Ignorer::load()?,
  })
}

fn load_config_file() -> Result<Vec<Regex>> {
  let mut result = vec![];
  let file = fs::File::open(CONFIG_FILE_NAME).map_err(|e| UserError::ConfigFileRead {
    file: CONFIG_FILE_NAME.into(),
    reason: e.to_string(),
  })?;
  for (i, line) in BufReader::new(file).lines().enumerate() {
    let line = line.unwrap();
    if line.is_empty() {
      // TODO: also ignore lines starting with # here
      continue;
    }
    match Regex::new(&line) {
      Ok(regex) => result.push(regex),
      Err(err) => {
        return Err(UserError::ConfigFileInvalidRegex {
          file: Utf8PathBuf::from(CONFIG_FILE_NAME),
          line: i,
          message: err.to_string(),
        });
      }
    }
  }
  Ok(result)
}
