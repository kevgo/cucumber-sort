use crate::prelude::*;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;

pub fn all() -> Result<Vec<Utf8PathBuf>> {
    let mut result = Vec::<Utf8PathBuf>::new();
    search_folder(".", &mut result)?;
    Ok(result)
}

fn search_folder(dir: impl AsRef<Utf8Path>, files: &mut Vec<Utf8PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir.as_ref()).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let path = Utf8Path::from_path(&entry_path).unwrap();
        if path.is_dir() {
            search_folder(&path, files)?;
        } else if path.extension() == Some("feature") {
            files.push(path.to_path_buf());
        }
    }
    Ok(())
}
