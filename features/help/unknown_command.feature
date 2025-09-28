Feature: unknown command

  Scenario:
    When I run "cucumber-sort zonk"
    Then it prints the error:
      """
      error: unrecognized subcommand 'zonk'

      Usage: cucumber-sort <COMMAND>

      For more information, try '--help'.
      """
    And the exit code is failure
