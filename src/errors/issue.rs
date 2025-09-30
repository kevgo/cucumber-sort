#[derive(Debug, Eq, PartialEq)]
pub struct Issue {
  pub line: usize,
  pub problem: String,
}

pub fn sort_issues(issues: &mut [Issue]) {
  issues.sort_by_key(|issue| issue.line);
}
