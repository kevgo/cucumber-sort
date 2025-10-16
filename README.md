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

### Step 1: collect all sortable steps

First, collect all Gherkin steps from your test suite into a file you can sort:

<pre type="shell/command" allow-error>
cucumber-sort check --record
</pre>

<a type="workspace/existing-file-with-content">

This creates file named **cucumber-sort.json**, which defines the expected step
order. Currently, it looks like this:

```json
{
  "$schema": "https://raw.githubusercontent.com/kevgo/cucumber-sort/refs/heads/main/docs/schema.json",
  "include": [],
  "exclude": [],
  "record": false,
  "fail-fast": false,
  "steps": [],
  "unknown-steps": [
    "^a mixing bowl$",
    "^apples$",
    "^butter$",
    "^cinnamon$",
    "^flour$"
  ]
}
```

</a>

Everything inside
<code type="workspace/existing-file-with-content" filename="cucumber-sort.json" partial-match>
"unknown-steps"</code> are Gherkin steps that _cucumber-sort_ can see but
doesn't yet know how to order.

### Step 2: arrange the steps in the desired order

<a type="workspace/new-file">

Edit **cucumber-sort.json** to arrange the steps in the order you want them to
appear in the recipes. For example:

```json
{
  "$schema": "https://raw.githubusercontent.com/kevgo/cucumber-sort/refs/heads/main/docs/schema.json",
  "include": [],
  "exclude": [],
  "record": false,
  "fail-fast": false,
  "steps": [
    // TOOLS
    "^a mixing bowl$",

    // BASE DOUGH
    "^flour$",
    "^butter$",

    // FRUITS
    "^apples$",

    // SPICES
    "^cinnamon$"
  ]
}
```

</a>

### Step 3: apply the new order

Format your feature files according to this new order:

<pre type="shell/command">
cucumber-sort format
</pre>

<a type="workspace/existing-file-with-content">

Now all recipes are consistently ordered. Here is how **apple_pie.feature**
looks after sorting:

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

The behavior is unchanged, but now your `.feature` files are consistent,
readable, and easier to maintain.

> [!TIP]
> To see a real-world example of using _cucumber-sort_ in production, check out
> the [Git Town codebase](https://github.com/git-town/git-town).

### Step 4: sort repetitive steps

Sometimes multiple steps interleave several times. As an example, creating
[laminated dough](https://en.wikipedia.org/wiki/Laminated_dough) requires to
repeatedly add layers of dough and butterj

<a type="workspace/new-file" filename="laminated_1.feature">

```cucumber
Feature: laminated dough

  Scenario: make the dough
    Given a mixing bowl
    Then fold the dough
    And add a layer of butter
    And chill in the fridge
    And fold the dough
    And add a layer of butter
    And chill in the fridge
    And fold the dough
    And add a layer of butter
    And sprinkle with cinnamon
```

</a>
<a type="workspace/copy-file" src="laminated_1.feature" dst="laminated_2.feature"></a>
<a type="workspace/copy-file" src="laminated_1.feature" dst="laminated_3.feature"></a>

A naive approach would be this step order:

<a type="workspace/new-file" filename="cucumber-sort.json">

```json
{
  "steps": [
    "a mixing bowl",
    "fold the dough",
    "add a layer of butter",
    "chill in the fridge",
    "sprinkle with cinnamon"
  ]
}
```

</a>

However, sorting steps this way would mess up the recipe:

<a type="shell/command" command="cucumber-sort format laminated_1.feature"></a>
<a type="workspace/existing-file-with-content" filename="laminated_1.feature">

```cucumber
Feature: laminated dough

  Scenario: make the dough
    Given a mixing bowl
    Then fold the dough
    And fold the dough
    And fold the dough
    And add a layer of butter
    And add a layer of butter
    And add a layer of butter
    And chill in the fridge
    And chill in the fridge
    And sprinkle with cinnamon
```

</a>

To sort this recipe properly, we need to tell _cucumber-sort_ to keep the steps
`fold the dough`, `add a layer of butter`, and `chill in the fridge` in the
order they occur:

<a type="workspace/new-file" filename="cucumber-sort.json">

```json
{
  "steps": [
    "a mixing bowl",
    [
      "fold the dough",
      "add a layer of butter",
      "chill in the fridge"
    ],
    "sprinkle with cinnamon"
  ]
}
```

</a>

<a type="shell/command" command="cucumber-sort format laminated_2.feature"></a>
<a type="workspace/compare-files" have="laminated_2.feature"
want="laminated_3.feature">

Now if we sort the file, it keeps the original order.

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

Generate the default configuration with:

<pre type="subcommand">
cucumber-sort init
</pre>

This command creates file
<b type="workspace/existing-file">cucumber-sort.json</b>. JSON-Schema is
available.

> [!TIP]
> See our own [cucumber-sort.json file](cucumber-sort.json) file for a working
> example.

## Commands

Format all `.feature` files according to your configured step order:

<pre type="subcommand">
cucumber-sort format
</pre>

Check whether your `.feature` files match the configured order:

<pre type="subcommand">
cucumber-sort check
</pre>

If you would like to add unknown steps to `cucumber-sort.json`, run:

<pre type="subcommand">
cucumber-sort check --record
</pre>

This appends unknown steps to the order file. Review the file and move them to
the right position.

If there are too many unknown steps, stop at the first file with issues:

<pre type="subcommand">
cucumber-stort check --fail-fast
</pre>
