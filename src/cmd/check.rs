use crate::sort::{self, Issue};
use crate::{config, find, gherkin, prelude::*};

pub fn check() -> Result<usize> {
    let config = config::load()?;
    let mut issues = Vec::<Issue>::new();
    for filepath in find::all()? {
        let gherkin = gherkin::load(&filepath)?;
        let sorted_file = sort::file(gherkin, &config, &filepath, &mut issues);
    }
    for issue in &issues {
        println!("{}:{}  {}", issue.file, issue.line, issue.problem);
    }
    Ok(issues.len())
}
