# cucumber-sort

- checks that steps are in the right order
- checks that scenarios don't contain duplicate steps?
- enforces the steps only within `Given`, `When`, and `Then` blocks
- relies that the user labels Given, When, and Then correctly, and that tooling enforces this

### Configuration file

```
[[one-of-these]]  # ensures exactly one of the nested expression is contained
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

- duplication of step definition regexes: once in the step definition, and again in the config file for this tool
  - that's probably okay, this is a linter
- creating the config file
