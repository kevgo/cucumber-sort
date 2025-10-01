Feature: check unknown steps

  Background: "step 3" is not defined in the config file
    Given file ".cucumber-sort-rc" with content:
      """
      step 1
      """
    And file "feature/one.feature" with content:
      """
      Feature: example

        Background:
          Given step 1
          And step 2
      """

  Scenario: run without recording
    When I run "cucumber-sort check"
    Then it prints:
      """
      feature/one.feature:5  unknown step: step 2
      """
    And the exit code is failure

  Scenario: run with recording
    When I run "cucumber-sort check --record"
    Then it prints:
      """
      feature/one.feature:5  unknown step: step 2
      """
    And the exit code is failure
    And file ".cucumber-sort-rc" now has content:
      """
      step 1

      # UNKNOWN STEPS
      step 2
      """
