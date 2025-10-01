pub fn insert_regex_placeholders(text: &str) -> String {
  let mut result = String::new();
  let mut chars = text.chars().peekable();

  while let Some(ch) = chars.next() {
    if ch == '"' {
      // Skip the opening quote and everything until the closing quote
      while let Some(inner_ch) = chars.next() {
        if inner_ch == '"' {
          break;
        }
      }
      // Replace the entire quoted string with .*
      result.push_str(".*");
    } else {
      result.push(ch);
    }
  }

  result
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
