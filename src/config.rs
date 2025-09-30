use crate::FeatureFinder;
use crate::errors::Result;
use crate::gherkin::Sorter;

pub struct Config {
  pub finder: FeatureFinder,
  pub sorter: Sorter,
}

pub fn load() -> Result<Config> {
  Ok(Config {
    finder: FeatureFinder::load()?,
    sorter: Sorter::load()?,
  })
}

pub fn create() -> Result<()> {
  FeatureFinder::create()?;
  Sorter::create()
}
