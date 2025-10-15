use cucumber_sort::config::Config;
use schemars::schema_for;
use std::fs;
use std::path::Path;

#[test]
fn export_schema() {
  // Generate the schema
  let schema = serde_json::to_string_pretty(&schema_for!(Config)).unwrap() + "\n";

  // Write schema to docs/schema.json
  let schema_path = Path::new("docs").join("schema.json");
  fs::write(&schema_path, schema).unwrap();
}
