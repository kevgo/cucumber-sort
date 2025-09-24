Feature: ordered steps

  Scenario:
    Given file ".cucumbersortrc":
      """
      one
      """
    And file "feature/one.feature":
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
    When I run cucumber-sort
    Then it prints:
      """
      Hello, world!
      """
