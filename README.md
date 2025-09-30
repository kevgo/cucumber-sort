# cucumber-sort

![build status](https://github.com/kevgo/cucumber-sort/actions/workflows/ci.yml/badge.svg)

This tool sorts steps in [Gherkin](https://cucumber.io/docs/gherkin/reference)
files according to an order that you define.

### Installation

The easiest way to execute this tool is through
[run-that-app](https://github.com/kevgo/run-that-app).

If you want to run the tool standalone, download and extract/install the
[latest release](https://github.com/kevgo/cucumber-sort/releases/latest).

You can also install from source:

- [install Rust](https://rustup.rs)
- clone the repo and cd into the directory
- compile the executable: `cargo install --locked --path .`

### Configuration file

```
a Git repo with origin
a local Git repo
I am outside a Git repo
the branches
the local branch is {string}
the commits
I ran {string}
I run {string}
I run {string} and enter into the dialog
```

### Challenges:

- duplication of step definition regexes: once in the step definition, and again
  in the config file for this tool
  - that's probably okay, this is a linter
- creating the config file
