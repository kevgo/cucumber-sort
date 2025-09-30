use crate::errors::Result;
use crate::filesystem::Globber;
use crate::gherkin::Sorter;

pub struct Config {
  pub sorter: Sorter,
  pub globber: Globber,
}

pub fn load() -> Result<Config> {
  Ok(Config {
    sorter: Sorter::load()?,
    globber: Globber::load()?,
  })
}
