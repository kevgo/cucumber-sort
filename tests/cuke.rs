use cucumber::{World, then, when};
use tokio::process::Command;

#[derive(Debug, Default, World)]
pub struct MyWorld {
    output: String,
}

#[when("I run cucumber-sort")]
async fn run_binary(world: &mut MyWorld) {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cucumber-sort"])
        .output()
        .await
        .unwrap();
    world.output = String::from_utf8(output.stdout).unwrap();
}

#[then(expr = "it prints {string}")]
async fn check_output(world: &mut MyWorld, expected: String) {
    assert_eq!(world.output.trim(), expected);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    MyWorld::run("features").await;
}
