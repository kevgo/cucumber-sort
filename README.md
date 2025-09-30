# cucumber-sort

![build status](https://github.com/kevgo/cucumber-sort/actions/workflows/ci.yml/badge.svg)

This tool sorts steps in [Cucumber](https://cucumber.io) files in an order that
you define.

### Installation

The easiest way to execute this tool is through
[run-that-app](https://github.com/kevgo/run-that-app):

```
rta cucumber-sort
```

If you want to run the tool standalone, download and extract/install the
[latest release](https://github.com/kevgo/cucumber-sort/releases/latest). You
can also install from source:

- [install Rust](https://rustup.rs)
- clone the repo and cd into the directory
- compile the executable: `cargo install --locked --path .`

### Configuration file

Run `cucumber-sort init` to create the configuration files.

File **.cucumber-sort-rc** contains the step names without Given/When/Then in
the order you want them to appear in the`.feature` files. You can use regular
expressions for placeholders. They only need to match the steps and don't need
to contain captures, since cucumber-sort doesn't capture any data out of
Cucumber steps.

File **.cucumber-sort-ignore** lists files that cucumber-sort should ignore. You
can use glob expressions in it.

### Fixing the order

To format your `.feature` files to have the correct step order, run:

```
cucumber-sort format
```

### Verifying the order

To verify that all your `.feature` files have the correct step order, run:

```
cucumber-sort check
```
