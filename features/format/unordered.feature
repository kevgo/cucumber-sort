Feature: format unordered steps

  Scenario: unordered step in a scenario
    Given file ".cucumbersortrc" with content:
      """
      file .* with content:
      step 2
      step 3
      step 4
      step 5
      file .* now has content:
      """
    And file "feature/one.feature" with content:
      """
      Feature: example
        Comment text
        describing the feature.

        Background:
          Given step 2
          And file "foo" with content:
            '''
            bar
            '''

        Scenario: result
          Then step 4
          And step 3

        # another comment

        Scenario: undo
          When step 5
          Then file "foo" now has content:
            '''
            baz
            '''
      """
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
    And file "feature/one.feature" now has content:
      """
      Feature: example
        Comment text
        describing the feature.

        Background:
          And file "foo" with content:
            '''
            bar
            '''
          Given step 2

        Scenario: result
          And step 3
          Then step 4

        # another comment

        Scenario: undo
          When step 5
          Then file "foo" now has content:
            '''
            baz
            '''
      """
