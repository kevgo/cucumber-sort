mod cli;
mod cmd;
mod config;
mod domain;
mod find;
mod gherkin;
mod prelude;
mod sort;

use cli::Command::{Check, Format, Help};
use prelude::*;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(count) => {
            if count == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(err) => {
            println!("{}", err);
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<usize> {
    match cli::load(env::args())? {
        Check => cmd::check(),
        Format => cmd::format(),
        Help => cmd::help(),
    }
}
