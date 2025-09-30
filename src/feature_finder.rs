use crate::errors::{Result, UserError};
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::io::ErrorKind;

/// the filename of the ignore file
const IGNORE_FILE_NAME: &str = ".cucumbersortignore";

/// Ignorer encapsulates the minutiae around ignoring file paths.
/// You give it an ignore config file, and it tells you whether
/// particular file paths are ignored according to it or not.
pub struct FeatureFinder {
  globs: Vec<glob::Pattern>,
}

impl FeatureFinder {
  /// loads a new instance from the default ignore file
  pub fn load() -> Result<FeatureFinder> {
    match fs::read_to_string(IGNORE_FILE_NAME) {
      Ok(text) => FeatureFinder::parse(&text, IGNORE_FILE_NAME.into()),
      Err(err) => match err.kind() {
        ErrorKind::NotFound => Ok(FeatureFinder { globs: vec![] }),
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
      if self.is_ignored(entry_path.as_str()) {
        continue;
      }
      result.push(entry_path.to_path_buf());
    }
    Ok(result)
  }

  /// indicates whether the given file path is ignored
  fn is_ignored(&self, file: &str) -> bool {
    for glob in &self.globs {
      if glob.matches(file) {
        return true;
      }
    }
    false
  }

  fn parse(config: &str, source: &Utf8Path) -> Result<FeatureFinder> {
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
    Ok(FeatureFinder { globs })
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
    let ignorer = super::FeatureFinder::parse(config, super::IGNORE_FILE_NAME.into()).unwrap();
    assert!(ignorer.is_ignored("features/unordered1.feature"));
    assert!(ignorer.is_ignored("features/unordered2.feature"));
    assert!(ignorer.is_ignored("features/weird1.feature"));
    assert!(ignorer.is_ignored("features/weird2.feature"));
    assert!(!ignorer.is_ignored("features/ordered.feature"));
  }

  mod parse {
    use crate::FeatureFinder;
    use crate::errors::UserError;
    use core::panic;

    #[test]
    fn correct() {
      let config = r#"
        feature/unordered*.feature
        feature/weird*.feature
      "#;
      FeatureFinder::parse(config, "somefile".into()).unwrap();
    }

    #[test]
    fn incorrect() {
      let config = r#"
feature/valid.feature
file[name
"#;
      let Err(UserError::IgnoreFileInvalidGlob { file, line, reason }) =
        FeatureFinder::parse(config, "somefile".into())
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
