use std::process::ExitCode;

use crate::sort::{self, Issue};
use crate::{config, find, gherkin, prelude::*};

pub fn check() -> Result<ExitCode> {
    let config = config::load()?;
    let mut issues = Vec::<Issue>::new();
    let mut exit_code = ExitCode::SUCCESS;
    for filepath in find::all()? {
        let gherkin = gherkin::load(&filepath)?;
        let sorted_file = sort::file(gherkin, &config, &filepath, &mut issues);

        for issue in &issues {
            exit_code = ExitCode::FAILURE;
            println!("{}:{}  {}", issue.file, issue.line, issue.problem);
        }
        issues.clear();
    }
    Ok(exit_code)
}
