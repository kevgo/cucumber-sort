# cucumber-sort

![build status](https://github.com/kevgo/cucumber-sort/actions/workflows/ci.yml/badge.svg)

**cucumber-sort** enforces a consistent step order in your
[Cucumber](https://cucumber.io) `.feature` files.

### Installation

The easiest way to run `cucumber-sort` is via
[run-that-app](https://github.com/kevgo/run-that-app):

```zsh
rta cucumber-sort
```

Other options:

- download the
  [latest release](https://github.com/kevgo/cucumber-sort/releases/latest) and
  install manually
- Build from source:
  - [Install Rust](https://rustup.rs)
  - Clone the repo and cd into it
  - Run:

    ```zsh
    cargo install --locked --path .
    ```

### Configuration file

Generate the default config files with:

```zsh
cucumber-sort init
```

This creates two files:

#### .cucumber-sort-rc

Defines the step order. Add step names (without `Given`/`When`/`Then`) in the
order you want them to appear in your `.feature` files.

- Supports regular expressions
- Regex only need to match the text, no captures required

> [!TIP]
> This is a helpful tip for the user. It can span multiple lines.

```tip
As an example, take a look at our own
[.cucumber-sort-rc file](.cucumber-sort-rc).
```

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

```
```
