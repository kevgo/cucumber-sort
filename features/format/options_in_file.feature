Feature: enable options through the opts file

  Scenario: unordered step in a scenario
    Given file "cucumber-sort.json" with content:
      """
      {
        "$schema": "https://raw.githubusercontent.com/kevgo/cucumber-sort/refs/heads/main/docs/schema.json",
        "include": [],
        "exclude": [],
        "record": true,
        "fail-fast": true,
        "steps": [
          "step 1"
        ],
        "unknown-steps": [
          "^another unknown step$",
          "^file \".*\" with content:$"
        ]
      }
      """
    And file "features/one.feature" with content:
      """
      Feature: example

        Scenario: steps out of order
          Then step 2
          And step 1
      """
    And file "features/two.feature" with content:
      """
      Feature: example

        Scenario: steps out of order
          Then step 2
          And step 1
      """
    When I run "cucumber-sort format"
    Then it prints:
      """
      features/one.feature:4  unknown step: step 2
      """
    And the exit code is failure
    And file "cucumber-sort.json" now has content:
      """
      {
        "$schema": "https://raw.githubusercontent.com/kevgo/cucumber-sort/refs/heads/main/docs/schema.json",
        "include": [],
        "exclude": [],
        "record": true,
        "fail-fast": true,
        "steps": [
          "step 1"
        ],
        "unknown-steps": [
          "^another unknown step$",
          "^file \".*\" with content:$",
          "^step 2$"
        ]
      }
      """
