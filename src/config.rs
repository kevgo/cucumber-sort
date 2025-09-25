use crate::prelude::*;
use camino::Utf8PathBuf;
use std::fs;
use std::io::{BufRead, BufReader};

pub const FILE_NAME: &str = "cucumbersortrc";

pub struct Config {
    pub steps: Vec<String>,
}

pub fn load() -> Result<Config> {
    let file = fs::File::open(FILE_NAME).map_err(|e| UserError::CannotReadConfigFile {
        file: Utf8PathBuf::from(FILE_NAME),
        reason: e.to_string(),
    })?;
    let reader = BufReader::new(file);
    Ok(Config {
        steps: reader.lines().into_iter().map(|e| e.unwrap()).collect(),
    })
}
