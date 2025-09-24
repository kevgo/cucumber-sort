mod check;
mod config;
mod domain;
mod find;
mod parse;
mod prelude;

use check::Issue;
use prelude::*;
use std::fs;
use std::io::BufReader;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("{}", err);
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<()> {
    let config = config::load()?;
    let mut issues = Vec::<Issue>::new();
    for file in find::all()? {
        let file = fs::File::open(&file).map_err(|e| UserError::CannotReadFile {
            filename: file,
            reason: e.to_string(),
        })?;
        let f2 = parse::gherkin(BufReader::new(file))?;
    }
    for issue in issues {
        println!("{}:{}  {}", issue.file, issue.line, issue.problem);
    }
    Ok(())
}
