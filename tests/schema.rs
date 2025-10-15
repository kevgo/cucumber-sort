use cucumber_sort::config::Config;
use schemars::schema_for;
use std::fs;
use std::path::Path;

#[test]
fn export_schema() {
  let schema = serde_json::to_string_pretty(&schema_for!(Config)).unwrap() + "\n";
  fs::write(&Path::new("docs").join("schema.json"), schema).unwrap();
}
