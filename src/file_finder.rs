use crate::config::Config;
use crate::errors::{Result, UserError};
use camino::{Utf8DirEntry, Utf8Path, Utf8PathBuf};

/// FileFinder encapsulates the logic for finding feature files.
/// It respects include and exclude patterns from the configuration.
pub struct FileFinder {
  include_globs: Vec<glob::Pattern>,
  exclude_globs: Vec<glob::Pattern>,
}

impl FileFinder {
  pub fn search_folder(&self, dir: impl AsRef<Utf8Path>) -> Result<Vec<Utf8PathBuf>> {
    let mut result = vec![];
    let entries: Vec<Utf8DirEntry> = dir
      .as_ref()
      .read_dir_utf8()
      .map_err(|err| UserError::DirRead {
        dir: dir.as_ref().to_path_buf(),
        message: err.to_string(),
      })?
      .collect::<std::result::Result<Vec<_>, _>>()
      .map_err(|err| UserError::FileRead {
        file: dir.as_ref().to_path_buf(),
        reason: err.to_string(),
      })?;
    let mut paths: Vec<&Utf8Path> = entries.iter().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
      let entry_path = path.strip_prefix(".").unwrap_or(path);
      if entry_path.is_dir() {
        result.extend(self.search_folder(entry_path)?);
        continue;
      }
      if entry_path.extension() != Some("feature") {
        continue;
      }
      if self.should_exclude(entry_path) {
        continue;
      }
      if !self.should_include(entry_path) {
        continue;
      }
      result.push(entry_path.to_path_buf());
    }
    Ok(result)
  }

  /// indicates whether the given file path should be excluded
  fn should_exclude(&self, file: &Utf8Path) -> bool {
    for glob in &self.exclude_globs {
      if glob.matches(file.as_str()) {
        return true;
      }
    }
    false
  }

  /// indicates whether the given file path should be included
  /// If no include patterns are specified, all files are included by default
  fn should_include(&self, file: &Utf8Path) -> bool {
    if self.include_globs.is_empty() {
      return true;
    }
    for glob in &self.include_globs {
      if glob.matches(file.as_str()) {
        return true;
      }
    }
    false
  }
}

impl TryFrom<&Config> for FileFinder {
  type Error = UserError;

  /// Creates a new FileFinder from the JSON configuration
  fn try_from(config: &Config) -> Result<Self> {
    let mut include_globs = vec![];
    for pattern in &config.include {
      let glob = glob::Pattern::new(pattern).map_err(|err| UserError::IgnoreFileInvalidGlob {
        file: crate::config::CONFIG_FILE_NAME.into(),
        line: 0,
        reason: format!("Invalid include glob pattern '{}': {}", pattern, err),
      })?;
      include_globs.push(glob);
    }

    let mut exclude_globs = vec![];
    for pattern in &config.exclude {
      let glob = glob::Pattern::new(pattern).map_err(|err| UserError::IgnoreFileInvalidGlob {
        file: crate::config::CONFIG_FILE_NAME.into(),
        line: 0,
        reason: format!("Invalid exclude glob pattern '{}': {}", pattern, err),
      })?;
      exclude_globs.push(glob);
    }
    Ok(FileFinder {
      include_globs,
      exclude_globs,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::config::Config;

  #[test]
  fn should_exclude_file() {
    let config = Config {
      exclude: vec![
        "features/unordered*.feature".to_string(),
        "features/weird*.feature".to_string(),
      ],
      ..Default::default()
    };
    let finder = super::FileFinder::try_from(&config).unwrap();
    assert!(finder.should_exclude("features/unordered1.feature".into()));
    assert!(finder.should_exclude("features/unordered2.feature".into()));
    assert!(finder.should_exclude("features/weird1.feature".into()));
    assert!(finder.should_exclude("features/weird2.feature".into()));
    assert!(!finder.should_exclude("features/ordered.feature".into()));
  }

  #[test]
  fn should_include_all_when_no_patterns() {
    let config = Config::default();
    let finder = super::FileFinder::try_from(&config).unwrap();
    assert!(finder.should_include("features/any.feature".into()));
    assert!(finder.should_include("features/test.feature".into()));
  }

  #[test]
  fn should_include_only_matching() {
    let config = Config {
      include: vec!["features/important*.feature".to_string()],
      ..Default::default()
    };
    let finder = super::FileFinder::try_from(&config).unwrap();
    assert!(finder.should_include("features/important1.feature".into()));
    assert!(finder.should_include("features/important2.feature".into()));
    assert!(!finder.should_include("features/other.feature".into()));
  }

  #[test]
  fn invalid_exclude_glob() {
    let config = Config {
      exclude: vec!["file[name".to_string()],
      ..Default::default()
    };
    let result = super::FileFinder::try_from(&config);
    assert!(result.is_err());
  }

  #[test]
  fn invalid_include_glob() {
    let config = Config {
      include: vec!["file[name".to_string()],
      ..Default::default()
    };
    let result = super::FileFinder::try_from(&config);
    assert!(result.is_err());
  }
}
