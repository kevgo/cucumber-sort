use crate::prelude::*;
use camino::Utf8Path;
use std::fs;
use std::io::ErrorKind;

/// the filename of the ignore file
pub const IGNORE_FILE_NAME: &str = ".cucumbersortignore";

/// Ignorer encapsulates the minutiae around ignoring file paths.
/// You give it an ignore config file, and it tells you whether
/// particular file paths are ignored according to it or not.
pub struct Ignorer {
  globs: Vec<glob::Pattern>,
}

impl Ignorer {
  pub fn parse(config: &str, source: &Utf8Path) -> Result<Ignorer> {
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
    Ok(Ignorer { globs })
  }

  pub fn load() -> Result<Ignorer> {
    match fs::read_to_string(IGNORE_FILE_NAME) {
      Ok(text) => Ignorer::parse(&text, IGNORE_FILE_NAME.into()),
      Err(err) => match err.kind() {
        ErrorKind::NotFound => Ok(Ignorer { globs: vec![] }),
        _ => Err(UserError::ConfigFileRead {
          file: IGNORE_FILE_NAME.into(),
          reason: err.to_string(),
        }),
      },
    }
  }

  pub fn is_ignored(&self, file: &str) -> bool {
    for glob in &self.globs {
      if glob.matches(file) {
        return true;
      }
    }
    false
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
    let ignorer = super::Ignorer::parse(config, super::IGNORE_FILE_NAME.into()).unwrap();
    assert!(ignorer.is_ignored("features/unordered1.feature"));
    assert!(ignorer.is_ignored("features/unordered2.feature"));
    assert!(ignorer.is_ignored("features/weird1.feature"));
    assert!(ignorer.is_ignored("features/weird2.feature"));
    assert!(!ignorer.is_ignored("features/ordered.feature"));
  }

  mod parse {
    use crate::filesystem::ignore_files::Ignorer;
    use crate::prelude::UserError;
    use core::panic;

    #[test]
    fn correct() {
      let config = r#"
        feature/unordered*.feature
        feature/weird*.feature
      "#;
      Ignorer::parse(config, "somefile".into()).unwrap();
    }

    #[test]
    fn incorrect() {
      let config = r#"
feature/valid.feature
file[name
"#;
      let Err(UserError::IgnoreFileInvalidGlob { file, line, reason }) =
        Ignorer::parse(config, "somefile".into())
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
