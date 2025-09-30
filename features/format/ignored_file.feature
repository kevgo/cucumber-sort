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
      .cucumbersortrc:1  unused regex: step 1
      .cucumbersortrc:2  unused regex: step 2
      """
    And the exit code is success
