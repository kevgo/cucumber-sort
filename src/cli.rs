use std::fs;

use camino::Utf8PathBuf;
use clap::Parser;

const FILENAME: &str = ".cucumber-sort-opts";

pub fn parse() -> Command {
  let args = std::env::args();
  match read_file() {
    Some(file_args) => Command::parse_from(args.chain(file_args)),
    None => Command::parse_from(args),
  }
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

fn read_file() -> Option<Vec<String>> {
  let Ok(text) = fs::read_to_string(FILENAME) else {
    return None;
  };
  let result = text
    .lines()
    .flat_map(|line| line.split_whitespace())
    .map(String::from)
    .collect();
  Some(result)
}
