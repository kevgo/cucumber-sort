use crate::prelude::*;
use std::env;

pub fn load(mut args: env::Args) -> Result<Command> {
    let _ = args.next(); // skip the command path
    match args.next() {
        Some(arg) => match arg.as_str() {
            "check" => Ok(Command::Check),
            "format" => Ok(Command::Format),
            _ => Err(UserError::UnknownCommand(arg)),
        },
        None => Ok(Command::Help),
    }
}

pub enum Command {
    Check,
    Format,
    Help,
}
