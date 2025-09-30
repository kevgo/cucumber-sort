Feature: ignoring files

  Scenario: unordered step in a scenario
    Given file ".cucumber-sort-rc" with content:
      """
      step 1
      step 2
      """
    And file ".cucumber-sort-ignore" with content:
      """
      features/unordered*.feature
      """
    And file "features/unordered.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          When step 2
          And step 1
      """
    When I run "cucumber-sort format"
    Then it prints:
      """
      .cucumber-sort-rc:1  unused regex: step 1
      .cucumber-sort-rc:2  unused regex: step 2
      """
    And the exit code is success
