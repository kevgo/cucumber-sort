Feature: format a single file

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
          When step 2
          Then step 1
      """
    And file "features/ordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps in order
          When step 1
          Then step 2
      """
    When I run "cucumber-sort format features/ordered.feature"
    Then it prints nothing
    And the exit code is success
    And file contents haven't changed
