Feature: ignoring files

  Scenario: unordered step in a scenario
    Given file ".cucumbersortrc" with content:
      """
      step 1
      step 2
      """
    And file ".cucumbersortignore" with content:
      """
      features/unordered*.feature
      """
    And file "features/ordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          When step 1
          And step 2
      """
    And file "features/unordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          When step 2
          And step 1
      """
    When I run "cucumber-sort check"
    Then it prints nothing
    And the exit code is success
