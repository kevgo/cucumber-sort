Feature: check ordered steps

  Scenario:
    Given file ".cucumbersortrc" with content:
      """
      step 1
      step 2
      step 3
      step 4
      step 5
      """
    And file "feature/one.feature" with content:
      """
      Feature: example

        Background:
          Given step 1
          And step 2
          When step 3

        Scenario: result
          Then step 4
          And step 5
      """
    When I run "cucumber-sort check"
    Then it prints nothing
    And the exit code is success
