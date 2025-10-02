use crate::errors::Result;
use crate::gherkin::Sorter;
use crate::{FileFinder, cli};

pub struct Config {
  pub finder: FileFinder,
  pub sorter: Sorter,
}

pub fn load() -> Result<Config> {
  Ok(Config {
    finder: FileFinder::load()?,
    sorter: Sorter::load()?,
  })
}

pub fn create() -> Result<()> {
  FileFinder::create()?;
  Sorter::create()?;
  cli::create()
}
