use cucumber_sort::config::Config;
use schemars::schema_for;
use std::fs;
use std::path::Path;

#[test]
fn export_schema() {
  // Generate the schema
  let schema = serde_json::to_string_pretty(&schema_for!(Config)).unwrap() + "\n";

  // Ensure docs directory exists
  let docs_dir = Path::new("docs");
  if !docs_dir.exists() {
    fs::create_dir(docs_dir).expect("Failed to create docs directory");
  }

  // Write schema to docs/schema.json
  let schema_path = docs_dir.join("schema.json");
  fs::write(&schema_path, schema).expect("Failed to write schema file");

  // Verify the file was created and is valid JSON
  let contents = fs::read_to_string(&schema_path).expect("Failed to read schema file");
  let parsed: serde_json::Value =
    serde_json::from_str(&contents).expect("Schema is not valid JSON");

  // Basic validation - ensure it's a JSON Schema
  assert!(parsed.get("$schema").is_some(), "Missing $schema field");
  assert!(
    parsed.get("properties").is_some(),
    "Missing properties field"
  );
}
