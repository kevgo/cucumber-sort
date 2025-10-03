# cucumber-sort

![build status](https://github.com/kevgo/cucumber-sort/actions/workflows/ci.yml/badge.svg)

**cucumber-sort** enforces a consistent step order in the `.feature` files of
your [Cucumber](https://cucumber.io) test suite. As an example, let's say you
have a database of executable cooking recipes for your robotic kitchen,
implemented in Cucumber. Here is file **apple_pie.feature** from it:

```cucumber
Given a bowl
When I add cinnamon
And I add apples
And I add butter
And I add flour
And I bake it
```

The recipe is technically correct. The "test" passes, it produces apple pie.
However, this recipe would be easier to reason about and compare with other
recipes if we always specified the basic ingredients first and then list the
condiments.

This is what cucumber-sort helps you with. It enforces a specific order of steps
in your Cucumber files. Let's have it collect all the steps used in our test
suite for us:

```
cucumber-sort check --record
```

This creates file `.cucumber-sort-order` with this content:

```sh
# UNKNOWN STEPS
a bowl
I add cinnamon
I add apples
I add butter
I add flour
I bake it
```

Now we sort the steps in this file the way we want them to occur in recipes:

```sh
# TOOLS
a bowl

# BASE DOUGH
I add flour
I add butter

# FRUITS
I add apples

# SPICES
I add cinnamon

# COOKINg
I bake it
```

Let's apply this new order:

```
cucumber-sort format
```

Now the steps in all our recipes follow this order. Here is our reordered
recipe:

```cucumber
Given a bowl
When I add flour
And I add butter
And I add apples
And I add cinnamon
And I bake it
```

The Cucumber suite works the same before and after, but now it's better
organized.

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

<pre type="subcommand">
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
