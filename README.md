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

<pre type="call">
cucumber-sort init
</pre>

This creates three files:

#### .cucumber-sort-order

Defines the step order. Add step names (without `Given`/`When`/`Then`) in the
order you want them to appear in your `.feature` files.

- Supports regular expressions
- Regex only need to match the text, no captures required

> [!TIP]
> Take a look at our own [.cucumber-sort-order file](.cucumber-sort-order) for
> an example config file.

#### .cucumber-sort-ignore

Contains glob patterns for files that `cucumber-sort` should ignore.

#### .cucumber-sort-opts

Contains cucumber-sort CLI arguments that you always want to enable.

### Usage

Format all `.feature` files to the configured step order:

<pre type="call">
cucumber-sort format
</pre>

Check whether `.feature` files already follow the configured order:

<pre type="call">
cucumber-sort check
</pre>

On the initial runs of the tool, you likely see unknown steps. Add them to
`.cucumber-sort-order`. To make this easier:

<pre type="call">
cucumber-sort check --record
</pre>

This appends unknown steps to the file. Just review the file and move the
unknown steps into the correct position.

If this finds too many unknown steps, you can stop at the first file with
failures:

<pre type="call">
cucumber-stort check --fail-fast
</pre>
