use crate::errors::Result;
use crate::filesystem::globber::Globber;
use camino::{Utf8Path, Utf8PathBuf};

pub fn find_matching(ignorer: &Globber) -> Result<Vec<Utf8PathBuf>> {
  search_folder(".", ignorer)
}

fn search_folder(dir: impl AsRef<Utf8Path>, ignorer: &Globber) -> Result<Vec<Utf8PathBuf>> {
  let mut result = vec![];
  for entry in dir.as_ref().read_dir_utf8().unwrap() {
    let entry = entry.unwrap();
    let entry_path = entry.path();
    if entry_path.is_dir() {
      result.extend(search_folder(entry_path, ignorer)?);
      continue;
    }
    if entry_path.extension() != Some("feature") {
      continue;
    }
    let entry_path = entry_path.strip_prefix(".").unwrap_or(entry_path);
    if ignorer.is_ignored(entry_path.as_str()) {
      continue;
    }
    result.push(entry_path.to_path_buf());
  }
  Ok(result)
}
