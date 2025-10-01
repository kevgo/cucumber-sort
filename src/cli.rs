use std::fs;

use camino::Utf8PathBuf;
use clap::Parser;

const FILENAME: &str = ".cucumber-sort-opts";

pub fn parse() -> Command {
  let mut args = std::env::args();
  if let Some(file_opts) = read_file() {
    args = args.chain(file_opts);
  }
  Command::parse_from(args)
}

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
pub enum Command {
  /// Check if Cucumber files are properly sorted
  Check {
    /// Stop at the first file that encounters problems
    #[arg(short, long)]
    fail_fast: bool,
    /// The file to check (optional)
    file: Option<Utf8PathBuf>,
    /// Record undefined steps in the config file
    #[arg(short, long)]
    record: bool,
  },
  /// Format Cucumber files by sorting them
  Format {
    /// Stop at the first file that encounters problems
    #[arg(short, long)]
    fail_fast: bool,
    /// The file to format (optional)
    file: Option<Utf8PathBuf>,
    /// Record undefined steps in the config file
    #[arg(short, long)]
    record: bool,
  },
  /// Create the configuration files
  Init,
}

fn read_file() -> Option<impl Iterator<Item = String>> {
  let Ok(text) = fs::read_to_string(FILENAME) else {
    return None;
  };
  // TODO: parse text into an iterator that can be chained into std::env::Args
  // and return it from this function.
  Some(result)
}
