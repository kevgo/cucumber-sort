use crate::prelude::*;
use camino::Utf8PathBuf;
use std::env;

pub fn load(mut args: env::Args) -> Result<Command> {
  let _ = args.next(); // skip the executable name
  match args.next() {
    Some(arg) => match arg.as_str() {
      "check" => Ok(Command::Check {
        file: args.next().map(Utf8PathBuf::from),
      }),
      "format" => Ok(Command::Format {
        file: args.next().map(Utf8PathBuf::from),
      }),
      _ => Err(UserError::UnknownCommand(arg)),
    },
    None => Ok(Command::Help),
  }
}

pub enum Command {
  Check { file: Option<Utf8PathBuf> },
  Format { file: Option<Utf8PathBuf> },
  Help,
}
