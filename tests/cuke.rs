use cucumber::{World, then, when};
use std::process::Command;

#[derive(Debug, Default, World)]
pub struct MyWorld {
    output: String,
}

#[when("I run cucumber-sort")]
async fn run_binary(world: &mut MyWorld) {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cucumber-sort"])
        .output()
        .unwrap();

    world.output = String::from_utf8(output.stdout).unwrap();
}

#[then(expr = r#"it prints {string}"#)]
async fn check_output(world: &mut MyWorld, expected: String) {
    assert_eq!(world.output.trim(), expected);
}

#[tokio::test]
async fn run_cucumber_tests() {
    MyWorld::run("features").await;
}
