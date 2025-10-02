Feature: format unknown steps

  Background:
    Given file ".cucumber-sort-order" with content:
      """
      step 1
      """
    And file "features/one.feature" with content:
      """
      Feature: example

        Background:
          Given step 1
          And step 3
      """

  Scenario: without record
    When I run "cucumber-sort format"
    Then it prints:
      """
      features/one.feature:5  unknown step: step 3
      """
    And the exit code is failure
    And file contents haven't changed

  Scenario: with record
    When I run "cucumber-sort format --record"
    Then it prints:
      """
      features/one.feature:5  unknown step: step 3
      """
    And the exit code is failure
    And file ".cucumber-sort-order" now has content:
      """
      step 1

      # UNKNOWN STEPS
      ^step 3$
      """
    And file "features/one.feature" hasn't changed

  Scenario: run with recording and existing marker
    Given file ".cucumber-sort-order" with content:
      """
      step 1

      # UNKNOWN STEPS
      ^another unknown step$
      ^file .* with content:$
      """
    When I run "cucumber-sort format --record"
    Then it prints:
      """
      features/one.feature:5  unknown step: step 3
      """
    And the exit code is failure
    And file ".cucumber-sort-order" now has content:
      """
      step 1

      # UNKNOWN STEPS
      ^step 3$
      """
