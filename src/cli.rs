use camino::Utf8PathBuf;
use clap::Parser;

pub fn parse() -> Command {
  Command::parse()
}

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
pub enum Command {
  /// Check if Cucumber files are properly sorted
  Check {
    /// The file to check (optional)
    file: Option<Utf8PathBuf>,
    /// Record undefined steps in the config file
    #[arg(long)]
    record: bool,
  },
  /// Format Cucumber files by sorting them
  Format {
    /// The file to format (optional)
    file: Option<Utf8PathBuf>,
    /// Record undefined steps in the config file
    #[arg(long)]
    record: bool,
  },
  /// Create the configuration files
  Init,
}
