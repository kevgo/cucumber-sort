# cucumber-sort

- checks that steps are in the right order
- checks that scenarios don't contain duplicate steps?
- enforces the steps only within `Given`, `When`, and `Then` blocks
- relies that the user labels Given, When, and Then correctly, and that tooling enforces this

### Configuration file

```
Given
  exactly-one:
    a Git repo with origin
    a local Git repo
    I am outside a Git repo
  zero-or-more:
    the branches
    the local branch is {string}
    the commits
  one-or-more
    I run ".*"
    I run ".*" and enter into the dialog
```

### Challenges:

- duplication of step definition regexes: once in the step definition, and again in the config file for this tool
- creating the config file
