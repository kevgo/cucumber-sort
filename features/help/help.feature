Feature: requesting help

  Scenario:
    When I run "cucumber-sort help"
    Then it prints:
      """
      Sorts steps in Gherkin files to match the order in cucumbersortrc.

      Available commands:

      check: verifies ordering of the Cucumber files
      format: fixes the order of Cucumber files
      """
    And the app terminates with a success code
