Feature: ordered steps

  Background:
    Given file ".cucumbersortrc:
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
