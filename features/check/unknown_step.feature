Feature: check unknown steps

  Scenario:
    Given file ".cucumbersortrc" with content:
      """
      step 1
      step 2
      """
    And file "feature/one.feature" with content:
      """
      Feature: example
      
        Background:
          Given step 1
          And step 3
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      feature/one.feature:5  unknown step: step 3
      """
    And the exit code is failure
