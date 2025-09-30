use crate::errors::{Result, UserError};
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::io::ErrorKind;

/// the filename of the ignore file
const IGNORE_FILE_NAME: &str = ".cucumber-sort-ignore";

const TEMPLATE: &str = r#"
# More info at https://github.com/kevgo/cucumber-sort
#
# This file lists files that cucumber-sort should ignore,
# using glob expressions.

# features/foo.feature
"#;

/// Ignorer encapsulates the minutiae around ignoring file paths.
/// You give it an ignore config file, and it tells you whether
/// particular file paths are ignored according to it or not.
pub struct FileFinder {
  globs: Vec<glob::Pattern>,
}

impl FileFinder {
  /// loads a new instance from the default ignore file
  pub fn load() -> Result<FileFinder> {
    match fs::read_to_string(IGNORE_FILE_NAME) {
      Ok(text) => FileFinder::parse(&text, IGNORE_FILE_NAME.into()),
      Err(err) => match err.kind() {
        ErrorKind::NotFound => Ok(FileFinder { globs: vec![] }),
        _ => Err(UserError::ConfigFileRead {
          file: IGNORE_FILE_NAME.into(),
          reason: err.to_string(),
        }),
      },
    }
  }

  pub fn search_folder(&self, dir: impl AsRef<Utf8Path>) -> Result<Vec<Utf8PathBuf>> {
    let mut result = vec![];
    for entry in dir.as_ref().read_dir_utf8().unwrap() {
      let entry = entry.unwrap();
      let entry_path = entry.path().strip_prefix(".").unwrap_or(entry.path());
      if entry_path.is_dir() {
        result.extend(self.search_folder(entry_path)?);
        continue;
      }
      if entry_path.extension() != Some("feature") {
        continue;
      }
      if self.is_ignored(entry_path) {
        continue;
      }
      result.push(entry_path.to_path_buf());
    }
    Ok(result)
  }

  pub fn create() -> Result<()> {
    fs::write(IGNORE_FILE_NAME, &TEMPLATE[1..]).map_err(|err| UserError::ConfigFileCreate {
      file: IGNORE_FILE_NAME.into(),
      message: err.to_string(),
    })
  }

  /// indicates whether the given file path is ignored
  fn is_ignored(&self, file: &Utf8Path) -> bool {
    for glob in &self.globs {
      if glob.matches(file.as_str()) {
        return true;
      }
    }
    false
  }

  fn parse(config: &str, source: &Utf8Path) -> Result<FileFinder> {
    let mut globs = vec![];
    for (i, line) in config.lines().enumerate() {
      if line.is_empty() || line.starts_with('#') {
        continue;
      }
      match glob::Pattern::new(line) {
        Ok(pattern) => globs.push(pattern),
        Err(err) => {
          return Err(UserError::IgnoreFileInvalidGlob {
            file: source.into(),
            line: i,
            reason: format!("Invalid glob pattern '{}': {}", line, err),
          });
        }
      }
    }
    Ok(FileFinder { globs })
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn is_ignored_file() {
    let config = r#"
features/unordered*.feature
features/weird*.feature
"#;
    let ignorer = super::FileFinder::parse(config, "config file name".into()).unwrap();
    assert!(ignorer.is_ignored("features/unordered1.feature".into()));
    assert!(ignorer.is_ignored("features/unordered2.feature".into()));
    assert!(ignorer.is_ignored("features/weird1.feature".into()));
    assert!(ignorer.is_ignored("features/weird2.feature".into()));
    assert!(!ignorer.is_ignored("features/ordered.feature".into()));
  }

  mod parse {
    use crate::FileFinder;
    use crate::errors::UserError;
    use core::panic;

    #[test]
    fn correct() {
      let config = r#"
        feature/one*.feature
        feature/two*.feature
      "#;
      FileFinder::parse(config, "somefile".into()).unwrap();
    }

    #[test]
    fn invalid_glob() {
      let config = r#"
feature/valid.feature
file[name
"#;
      let Err(UserError::IgnoreFileInvalidGlob { file, line, reason }) =
        FileFinder::parse(config, "somefile".into())
      else {
        panic!()
      };
      assert_eq!(file, "somefile");
      assert_eq!(line, 2);
      assert_eq!(
        reason,
        "Invalid glob pattern 'file[name': Pattern syntax error near position 4: invalid range pattern"
      );
    }
  }
}
