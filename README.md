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
> Take a look at our own [.cucumber-sort-rc file](.cucumber-sort-rc) for an
> example config file.

#### .cucumber-sort-ignore

Contains glob patterns for files that `cucumber-sort` should ignore.

### Usage

Format all `.feature` files to the configured step order:

```zsh
cucumber-sort format
```

Check whether `.feature` files already follow the configured order:

```zsh
cucumber-sort check
```

On the initial runs of the tool, you likely see unknown steps. Add them to
`.cucumber-sort-rc`. To make this easier:

```zsh
cucumber-sort check --record
```

This appends unknown steps to the file. Just review the file and move the
unknown steps into the correct position.
