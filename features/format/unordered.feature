Feature: format unordered steps

  Scenario: unordered step in a scenario
    Given file ".cucumber-sort-rc" with content:
      """
      file .* with content:
      step 2
      step 3
      step 4
      step 5
      file .* now has content:
      """
    And file "features/one.feature" with content:
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
    And file "features/one.feature" now has content:
      """
      Feature: example
        Comment text
        describing the feature.

        Background:
          Given file "foo" with content:
            '''
            bar
            '''
          And step 2

        Scenario: result
          Then step 3
          And step 4

        # another comment

        Scenario: undo
          When step 5
          Then file "foo" now has content:
            '''
            baz
            '''
      """
