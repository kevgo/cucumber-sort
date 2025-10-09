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
    #[command(flatten)]
    flags: Flags,

    /// The file to process (optional)
    file: Option<Utf8PathBuf>,
  },
  /// Format Cucumber files by sorting them
  Format {
    #[command(flatten)]
    flags: Flags,

    /// The file to process (optional)
    file: Option<Utf8PathBuf>,
  },
  /// Create the configuration file
  Init,
}

#[derive(Parser)]
pub struct Flags {
  /// Stop at the first file that encounters problems
  #[arg(short, long)]
  pub fail_fast: bool,

  /// Record undefined steps in the config file
  #[arg(short, long)]
  pub record: bool,
}
