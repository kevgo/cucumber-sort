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
    When I run "cucumber-sort check"
    Then it prints:
      """
      features/unordered.feature:4  expected When step 1 but found When step 2
      features/unordered.feature:5  expected And step 2 but found And step 1
      """
    And the exit code is failure
