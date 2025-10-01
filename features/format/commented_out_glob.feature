Feature: ignoring globs

  Scenario: comment out a regex
    Given file ".cucumber-sort-order" with content:
      """
      step 1
      step 2
      """
    And file ".cucumber-sort-ignore" with content:
      """
      # features/unordered.feature
      """
    And file "features/unordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          When step 2
          And step 1
      """
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
    And file "features/unordered.feature" now has content:
      """
      Feature: example
      
        Scenario: steps out of order
          When step 1
          And step 2
      """
