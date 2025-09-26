use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use tokio::fs;
use tokio::process::Command;

#[derive(Debug, World)]
pub struct MyWorld {
  /// the directory in which the test executes
  pub dir: camino_tempfile::Utf8TempDir,

  /// what the binary printed when running
  output: Option<String>,
}

impl Default for MyWorld {
  fn default() -> Self {
    Self {
      dir: camino_tempfile::tempdir().unwrap(),
      output: None,
    }
  }
}

#[given(expr = "file {string}:")]
async fn file(world: &mut MyWorld, step: &Step, filename: String) {
  let filepath = world.dir.path().join(filename);
  if let Some(parent) = filepath.parent() {
    fs::create_dir_all(parent).await.unwrap();
  }
  let content = step.docstring.as_ref().unwrap();
  fs::write(filepath, content).await.unwrap();
}

#[when("I run cucumber-sort")]
async fn run_binary(world: &mut MyWorld) {
  let output = Command::new("cargo")
    .args(&["run", "--bin", "cucumber-sort", "check"])
    .output()
    .await
    .unwrap();
  world.output = Some(String::from_utf8(output.stdout).unwrap());
}

#[then(expr = "it prints:")]
async fn check_output(world: &mut MyWorld, step: &Step) {
  let want = step.docstring.as_ref().unwrap();
  let Some(have) = world.output.take() else {
    panic!("no output captured");
  };
  assert_eq!(have.trim(), want.trim());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  MyWorld::run("features").await;
}
