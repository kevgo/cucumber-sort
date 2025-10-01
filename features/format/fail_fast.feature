Feature: check unordered steps while failing fast

  Scenario: unordered step in a scenario
    Given file ".cucumber-sort-rc" with content:
      """
      step 1
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
    When I run "cucumber-sort format --fail-fast"
    Then it prints:
      """
      features/two.feature:4  unknown step: step 2
      """
    And the exit code is failure
    And file contents haven't changed
