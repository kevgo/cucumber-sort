use camino::Utf8PathBuf;

pub fn block(lines: String, number: usize, issues: &mut Vec<Issue>) {
    //
}

pub struct Issue {
    pub file: Utf8PathBuf,
    pub line: usize,
    pub problem: String,
}
