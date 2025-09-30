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
      match glob::Pattern::new(&line) {
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

  pub fn is_ignored(&self, file: &Utf8Path) -> bool {
    for glob in &self.globs {
      if glob.matches(file.as_str()) {
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
    feature/unordered*.feature
    feature/weird*.feature
    "#;
    let ignorer = super::Ignorer::parse(config, super::IGNORE_FILE_NAME.into()).unwrap();
    assert!(ignorer.is_ignored("features/unordered1.feature".into()));
    assert!(ignorer.is_ignored("features/unordered2.feature".into()));
    assert!(ignorer.is_ignored("features/weird1.feature".into()));
    assert!(ignorer.is_ignored("features/weird2.feature".into()));
    assert!(!ignorer.is_ignored("features/ordered.feature".into()));
  }

  mod parse {
    use crate::filesystem::ignore_files::Ignorer;

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
        file[name
      "#;
      Ignorer::parse(config, "somefile".into()).unwrap();
    }
  }
}
