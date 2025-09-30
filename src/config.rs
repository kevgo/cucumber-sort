use crate::errors::Result;
use crate::filesystem::Ignorer;
use crate::gherkin::Sorter;

pub struct Config {
  pub sorter: Sorter,
  pub ignorer: Ignorer,
}

pub fn load() -> Result<Config> {
  Ok(Config {
    sorter: Sorter::load()?,
    ignorer: Ignorer::load()?,
  })
}
