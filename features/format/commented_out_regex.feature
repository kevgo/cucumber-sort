Feature: ignoring regexes

  Scenario: comment out a regex
    Given file ".cucumbersortrc" with content:
      """
      step 1
      step 2
      # step 3
      """
    And file "features/ordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          When step 1
          And step 2
      """
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
