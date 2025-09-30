use crate::FileFinder;
use crate::errors::Result;
use crate::gherkin::Sorter;

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
  Sorter::create()
}
