Feature: ordered steps

  Scenario:
    Given file ".cucumbersortrc":
      """
      step 1
      step 2
      """
    And file "feature/one.feature":
      """
      Feature: example
      
        Scenario: result
          Then step 2
          And step 1
      """
    When I run cucumber-sort
    Then it prints:
      """
      feature/one.feature:5  "step 1" should not come after "step 2"
      """
