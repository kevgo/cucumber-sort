Feature: check unordered steps

  Scenario: unordered step in a scenario
    Given file ".cucumber-sort-order" with content:
      """
      step 1
      step 2
      """
    And file "features/one.feature" with content:
      """
      Feature: example

        Scenario: steps out of order
          Then step 2
          And step 1
      """
    And file "features/two.feature" with content:
      """
      Feature: example

        Scenario: steps out of order
          Then step 2
          And step 1
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      features/one.feature:4  expected Then step 1 but found Then step 2
      features/one.feature:5  expected And step 2 but found And step 1
      features/two.feature:4  expected Then step 1 but found Then step 2
      features/two.feature:5  expected And step 2 but found And step 1
      """
    And the exit code is failure
