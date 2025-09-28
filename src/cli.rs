use camino::Utf8PathBuf;
use clap::Parser;

pub fn parse() -> Command {
  Command::parse()
}

#[derive(Parser)]
pub enum Command {
  /// Check if Cucumber files are properly sorted
  Check {
    /// The file to check (optional)
    file: Option<Utf8PathBuf>,
  },
  /// Format Cucumber files by sorting them
  Format {
    /// The file to format (optional)
    file: Option<Utf8PathBuf>,
  },
}
