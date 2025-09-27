Feature: format unknown steps

  Scenario:
    Given file ".cucumbersortrc":
      """
      step 1
      step 2
      """
    And file "feature/one.feature":
      """
      Feature: example

        Background:
          Given step 1
          And step 3
      """
    When I run "cucumber-sort format"
    Then it prints:
      """
      feature/one.feature:5  unknown step: step 3
      """
    And the app terminates with a non-zero exit code
