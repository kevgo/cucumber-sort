use crate::prelude::*;
use camino::{Utf8Path, Utf8PathBuf};

pub fn all() -> Result<Vec<Utf8PathBuf>> {
  let mut result = Vec::<Utf8PathBuf>::new();
  search_folder(".", &mut result)?;
  Ok(result)
}

fn search_folder(dir: impl AsRef<Utf8Path>, files: &mut Vec<Utf8PathBuf>) -> Result<()> {
  for entry in dir.as_ref().read_dir_utf8().unwrap() {
    let entry = entry.unwrap();
    let entry_path = entry.path();
    if entry_path.is_dir() {
      search_folder(entry_path, files)?;
    } else if entry_path.extension() == Some("feature") {
      files.push(entry_path.strip_prefix(".").unwrap().to_path_buf());
    }
  }
  Ok(())
}
