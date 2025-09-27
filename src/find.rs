use crate::prelude::*;
use camino::{Utf8Path, Utf8PathBuf};

pub fn all() -> Result<Vec<Utf8PathBuf>> {
  search_folder(".")
}

fn search_folder(dir: impl AsRef<Utf8Path>) -> Result<Vec<Utf8PathBuf>> {
  let mut result = vec![];
  for entry in dir.as_ref().read_dir_utf8().unwrap() {
    let entry = entry.unwrap();
    let entry_path = entry.path();
    if entry_path.is_dir() {
      result.extend(search_folder(entry_path)?);
    } else if entry_path.extension() == Some("feature") {
      result.push(entry_path.strip_prefix(".").unwrap().to_path_buf());
    }
  }
  Ok(result)
}
