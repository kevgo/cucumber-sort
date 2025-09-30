use crate::FeatureFinder;
use crate::errors::Result;
use crate::gherkin::Sorter;

pub struct Config {
  pub sorter: Sorter,
  pub globber: FeatureFinder,
}

pub fn load() -> Result<Config> {
  Ok(Config {
    sorter: Sorter::load()?,
    globber: FeatureFinder::load()?,
  })
}
