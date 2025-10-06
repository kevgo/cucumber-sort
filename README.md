# cucumber-sort

![build status](https://github.com/kevgo/cucumber-sort/actions/workflows/ci.yml/badge.svg)

**cucumber-sort** enforces a consistent order of steps across the `.feature`
files in your [Cucumber](https://cucumber.io) test suite.

## Example

Imagine you write an end-to-end test for your robotic kitchen, of course in
Cucumber. One of the tests entails the kitchen baking an apple pie from scratch.
Here is the (simplified) feature for that.

<a type="workspace/new-file" filename="apple_pie.feature">

```cucumber
Feature: apple pie

  Scenario: make the dough
    Given a mixing bowl
    And cinnamon
    And apples
    And butter
    And flour
```

</a>

This scenario works: it produces an apple pie. But it's not well organized.
Recipes would be easier to read and compare if they always listed the basic
ingredients first and then the optional ones.

That's exactly what _cucumber-sort_ helps with. It enforces a predictable,
project-wide order of steps in your Cucumber files.

### Step 1: record all steps

First, collect all Gherkin steps from your test suite into a file you can sort:

<pre type="shell/command" allow-error>
cucumber-sort check --record
</pre>

<a type="workspace/existing-file-with-content">

This creates file named **.cucumber-sort-order**, which defines the expected
step order. Currently, it looks like this:

```sh
# UNKNOWN STEPS
^a mixing bowl$
^apples$
^butter$
^cinnamon$
^flour$
```

</a>

Everything below
<code type="workspace/existing-file-with-content" filename=".cucumber-sort-order" partial-match>#
UNKNOWN STEPS</code> are Gherkin steps that _cucumber-sort_ can see but doesn't
know how to order. Let's arrange the steps in this file the way we want them to
occur in our recipes. <a type="workspace/new-file">We change file
**.cucumber-sort-order** to look like this:

```sh
# TOOLS
a mixing bowl

# BASE DOUGH
flour
butter

# FRUITS
apples

# SPICES
cinnamon
```

</a>

Let's apply this new order:

<pre type="shell/command">
cucumber-sort format
</pre>

<a type="workspace/existing-file-with-content">

Now the steps in all our recipes follow this order. Here is how file
**apple_pie.feature** from above looks like now, ordered by cucumber-sort:

```cucumber
Feature: apple pie

  Scenario: make the dough
    Given a mixing bowl
    And flour
    And butter
    And apples
    And cinnamon
```

</a>

Our recipe database works the same before and after, but now it's organized more
consistently.

> [!TIP]
> To see a real-world example of how _cucumber-sort_ is used in production,
> check out the [Git Town codebase](https://github.com/git-town/git-town).

## Installation

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

## Configuration file

Generate the default config files with:

<pre type="subcommand">
cucumber-sort init
</pre>

This creates three files:

### .cucumber-sort-order

Defines the step order. Add step names (without `Given`/`When`/`Then`) in the
order you want them to appear in your `.feature` files.

- Supports regular expressions
- Regex only need to match the text, no captures required

> [!TIP]
> Take a look at our own [.cucumber-sort-order file](.cucumber-sort-order) for
> an example config file.

### .cucumber-sort-ignore

Contains glob patterns for files that `cucumber-sort` should ignore.

### .cucumber-sort-opts

Contains cucumber-sort CLI arguments that you always want to enable.

## Usage

Format all `.feature` files to the configured step order:

<pre type="subcommand">
cucumber-sort format
</pre>

Check whether `.feature` files already follow the configured order:

<pre type="subcommand">
cucumber-sort check
</pre>

On the initial runs of the tool, you likely see unknown steps. Add them to
`.cucumber-sort-order`. To make this easier:

<pre type="subcommand">
cucumber-sort check --record
</pre>

This appends unknown steps to the file. Just review the file and move the
unknown steps into the correct position.

If this finds too many unknown steps, you can stop at the first file with
failures:

<pre type="subcommand">
cucumber-stort check --fail-fast
</pre>
