Feature: ordered steps

  @this
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
          And step 2
          When step 3
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      ./feature/one.feature:6  unknown step: step 3
      """
    And it fails
