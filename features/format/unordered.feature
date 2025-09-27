Feature: format unordered steps

  Scenario: unordered step in a scenario
    Given file ".cucumbersortrc":
      """
      step 1
      step 2
      step 3
      step 4
      step 5
      step 6
      """
    And file "feature/one.feature":
      """
      Feature: example
        Comment text
        describing the feature.

        Background:
          Given step 2
          And step 1

        Scenario: result
          Then step 4
          And step 3

        # another comment

        Scenario: undo
          When step 6
          Then step 5
      """
    When I run "cucumber-sort format"
    Then it prints:
      """

      """
    And the app terminates with a success code
    And file "feature/one.feature" now has content:
      """
      Feature: example
        Comment text
        describing the feature.

        Background:
          And step 1
          Given step 2

        Scenario: result
          And step 3
          Then step 4

        # another comment

        Scenario: undo
          Then step 5
          When step 6
      """
