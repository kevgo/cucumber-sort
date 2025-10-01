# cucumber-sort

![build status](https://github.com/kevgo/cucumber-sort/actions/workflows/ci.yml/badge.svg)

This tool organizes steps in [Cucumber](https://cucumber.io) files in an order
that you define.

### Installation

The easiest way to execute this tool is through
[run-that-app](https://github.com/kevgo/run-that-app):

```
rta cucumber-sort
```

You can also download the
[latest release](https://github.com/kevgo/cucumber-sort/releases/latest) and
extract/install manually. Or install from source:

- [install Rust](https://rustup.rs)
- clone the repo and cd into the cloned directory
- compile the executable: `cargo install --locked --path .`

### Configuration file

To create the configuration files for cucumber-sort, run:

```
cucumber-sort init
```

This creates two files:

#### .cucumber-sort-rc

Populate this file with the step names (without `Given`/`When`/`Then`) in the
order you want them to appear in the`.feature` files.

You can use regular expressions for placeholders. These regular expressions only
need to match the steps. They don't need to contain captures, since
cucumber-sort doesn't capture any data out of Cucumber steps.

As an example, take a look at our own
[.cucumber-sort-rc file](.cucumber-sort-rc).

#### .cucumber-sort-ignore

Populate this file with glob expressions that describe files that cucumber-sort
should ignore.

### Usage

To format your `.feature` files to have the correct step order, run:

```
cucumber-sort format
```

To verify that all your `.feature` files have the correct step order, run:

```
cucumber-sort check
```

Initially you'll encounter unknown steps. You need to add them to
**.cucumber-sort-rc**. To make this easy, run `cucumber-sort check --record`.
This adds the unknown steps to the file. All you need to do is review that file
and move the unknown steps to the correct position.
