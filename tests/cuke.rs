use std::env;
use std::process::ExitStatus;

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

  /// exit status of the binary
  exit_status: Option<ExitStatus>,
}

impl Default for MyWorld {
  fn default() -> Self {
    Self {
      dir: camino_tempfile::tempdir().unwrap(),
      output: None,
      exit_status: None,
    }
  }
}

#[given(expr = "file {string}:")]
async fn file(world: &mut MyWorld, step: &Step, filename: String) {
  let filepath = world.dir.path().join(filename);
  if let Some(parent) = filepath.parent()
    && parent != world.dir.path()
  {
    fs::create_dir_all(parent).await.unwrap();
  }
  let content = step.docstring.as_ref().unwrap();
  fs::write(filepath, content).await.unwrap();
}

#[when(expr = "I run {string}")]
async fn run_binary(world: &mut MyWorld, command: String) {
  let mut cmd_parts = command.split(' ');
  let mut binary_name = cmd_parts.next().unwrap().to_string();
  if binary_name == "cucumber-sort" {
    let cwd = env::current_dir().unwrap();
    binary_name = cwd
      .join("target")
      .join("release")
      .join("cucumber-sort")
      .to_string_lossy()
      .to_string();
  }
  let output = Command::new(binary_name)
    .args(cmd_parts)
    .current_dir(world.dir.path())
    .output()
    .await
    .unwrap();
  if !output.stderr.is_empty() {
    panic!(
      "running \"{command}\" produced unexpected STDERR output: {}",
      str::from_utf8(&output.stderr).unwrap()
    );
  }
  world.output = Some(String::from_utf8(output.stdout).unwrap());
  world.exit_status = Some(output.status);
}

#[then("it prints:")]
async fn it_prints(world: &mut MyWorld, step: &Step) {
  let want = step.docstring.as_ref().unwrap();
  let Some(have) = world.output.take() else {
    panic!("no output captured");
  };
  assert_eq!(have.trim(), want.trim());
}

#[then("it prints nothing")]
async fn prints_nothing(world: &mut MyWorld) {
  let Some(have) = &world.output else {
    panic!("no command run");
  };
  assert_eq!(have, "");
}

#[then("it succeeds")]
async fn succeeds(world: &mut MyWorld) {
  let Some(have) = &world.exit_status else {
    panic!("no command run");
  };
  if !have.success() {
    panic!(
      "expected success but received exit code {}",
      have.code().unwrap()
    );
  }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  MyWorld::run("features").await;
}
