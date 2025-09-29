use camino::Utf8PathBuf;
use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use std::env;
use std::path::PathBuf;
use std::process::ExitStatus;
use tokio::fs;
use tokio::process::Command;

#[derive(Debug, World)]
pub struct MyWorld {
  /// the directory in which the test executes
  dir: camino_tempfile::Utf8TempDir,

  /// all created files and their content
  files: Vec<(Utf8PathBuf, String)>,

  /// what the binary printed to STDOUT when running
  stdout: Option<String>,

  /// what the binary printed to STDERR when running
  stderr: Option<String>,

  /// exit status of the binary
  exit_status: Option<ExitStatus>,
}

impl Default for MyWorld {
  fn default() -> Self {
    Self {
      dir: camino_tempfile::tempdir().unwrap(),
      files: vec![],
      stdout: None,
      stderr: None,
      exit_status: None,
    }
  }
}

const NO_COMMAND_RUN: &str = "no command run";

#[then("file contents haven't changed")]
async fn files_not_changed(world: &mut MyWorld) {
  for (filepath, want_content) in &world.files {
    let have_content = fs::read_to_string(filepath).await.unwrap();
    let have_trimmed = have_content.trim();
    if *want_content != have_trimmed {
      pretty::assert_eq!(*want_content, have_trimmed);
      panic!("file {filepath} has unexpected content");
    }
  }
}

#[given(expr = "file {string} with content:")]
async fn create_file(world: &mut MyWorld, step: &Step, filename: String) {
  let filepath = world.dir.path().join(filename);
  let content = step.docstring.as_ref().unwrap().trim();
  let fixed_content = content.replace("\\\"", "\"");
  if let Some(parent) = filepath.parent()
    && parent != world.dir.path()
  {
    fs::create_dir_all(parent).await.unwrap();
  }
  fs::write(&filepath, fixed_content).await.unwrap();
  world.files.push((filepath, content.to_string()));
}

#[then(expr = "file {string} now has content:")]
async fn verify_file(world: &mut MyWorld, step: &Step, filename: String) {
  let filepath = world.dir.path().join(filename);
  let want = step.docstring.as_ref().unwrap().trim();
  let have = fs::read_to_string(filepath).await.unwrap();
  pretty::assert_eq!(want, have.trim());
}

#[when(expr = "I run {string}")]
async fn run_binary(world: &mut MyWorld, command: String) {
  let mut cmd_parts = command.split(' ');
  let mut executable = PathBuf::from(cmd_parts.next().unwrap().to_string());
  if executable.to_string_lossy() == "cucumber-sort" {
    let cwd = env::current_dir().unwrap();
    executable = cwd.join("target").join("debug").join("cucumber-sort");
  }
  let output = Command::new(executable)
    .args(cmd_parts)
    .current_dir(world.dir.path())
    .output()
    .await
    .unwrap();
  world.stdout = Some(String::from_utf8(output.stdout).unwrap());
  world.stderr = Some(String::from_utf8(output.stderr).unwrap());
  world.exit_status = Some(output.status);
}

#[then("it prints:")]
async fn it_prints(world: &mut MyWorld, step: &Step) {
  let want = step.docstring.as_ref().unwrap();
  let have = world.stdout.as_ref().expect(NO_COMMAND_RUN);
  let stripped = strip_ansi_escapes::strip_str(have);
  pretty::assert_eq!(want.trim(), stripped.trim());
}

#[then("it prints the error:")]
async fn it_prints_error(world: &mut MyWorld, step: &Step) {
  let want = step.docstring.as_ref().unwrap();
  let have = world.stderr.as_ref().expect(NO_COMMAND_RUN);
  let stripped = strip_ansi_escapes::strip_str(have);
  pretty::assert_eq!(want.trim(), stripped.trim());
}

#[then("it prints nothing")]
async fn prints_nothing(world: &mut MyWorld) {
  let have = &world.stdout.as_ref().expect(NO_COMMAND_RUN);
  pretty::assert_eq!(*have, "");
}

#[then("the exit code is success")]
async fn succeeds(world: &mut MyWorld) {
  let have = &world.exit_status.as_ref().expect(NO_COMMAND_RUN);
  if !have.success() {
    panic!(
      "expected success but received exit code {}",
      have.code().unwrap()
    );
  }
}

#[then("the exit code is failure")]
async fn fails(world: &mut MyWorld) {
  let have = &world.exit_status.as_ref().expect(NO_COMMAND_RUN);
  if have.success() {
    panic!("expected failure but the app succeeded");
  }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  MyWorld::run("features").await;
}
