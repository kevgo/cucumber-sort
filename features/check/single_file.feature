Feature: check a single file

  Scenario: unordered step in a scenario
    Given file ".cucumber-sort-rc" with content:
      """
      step 1
      step 2
      """
    And file "features/unordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          Then step 2
          And step 1
      """
    And file "features/ordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps in order
          Then step 1
          And step 2
      """
    When I run "cucumber-sort check features/ordered.feature"
    Then it prints nothing
    And the exit code is success
