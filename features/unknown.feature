Feature: unknown command

  Scenario:
    When I run "cucumber-sort zonk"
    Then it prints:
      """
      unknown command: zonk

      Available commands:

      check: verifies ordering of the Cucumber files
      format: fixes the order of Cucumber files
      """
    And the app terminates with a non-zero exit code
