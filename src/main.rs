mod config;
mod domain;
mod find;
mod gherkin;
mod prelude;
mod sort;

use prelude::*;
use sort::Issue;
use std::fs;
use std::io::BufReader;
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
    let config = config::load()?;
    let mut issues = Vec::<Issue>::new();
    for file in find::all()? {
        let file = fs::File::open(&file).map_err(|e| UserError::CannotReadFile {
            filename: file,
            reason: e.to_string(),
        })?;
        // let gherkin = gherkin::file(BufReader::new(file))?;
        // sort::file(gherkin, &config, &mut issues);
    }
    for issue in &issues {
        println!("{}:{}  {}", issue.file, issue.line, issue.problem);
    }
    Ok(issues.len())
}
