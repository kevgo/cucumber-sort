Feature: check unordered steps

  Scenario: unordered step in a scenario
    Given file ".cucumbersortrc":
      """
      step 1
      step 2
      """
    And file "feature/one.feature":
      """
      Feature: example
      
        Scenario: steps out of order
          Then step 2
          And step 1
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      feature/one.feature:4  expected And step 1 but found Then step 2
      feature/one.feature:5  expected Then step 2 but found And step 1
      """
    And the exit code is failure
