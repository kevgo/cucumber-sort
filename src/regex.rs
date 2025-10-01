pub fn insert_regex_placeholders(text: &str) -> String {
  text.into()
}

#[cfg(test)]
mod tests {

  #[test]
  fn insert_regex_placeholders() {
    let tests = vec![
      // no captures
      ("a foo walks into a bar", "a foo walks into a bar"),
      // one capture
      (
        "file \"foo.feature\" contains a bar",
        "file .* contains a bar",
      ),
      // multiple captures
      (
        "file \"foo.feature\" contains \"bar\"",
        "file .* contains .*",
      ),
    ];
    for (give, want) in tests {
      let have = super::insert_regex_placeholders(give);
      pretty::assert_eq!(want, have);
    }
  }
}
