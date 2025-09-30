Feature: format unordered steps

  Scenario: unordered step in a scenario
    Given file ".cucumbersortrc" with content:
      """
      file .* with content:
      step 1
      file .* now has content:
      """
    And file "feature/one.feature" with content:
      """
      Feature: example
      
        Scenario: test
          Given step 1
          And file "foo" with content:
            '''
            bar
            '''
      """
    When I run "cucumber-sort format"
    Then it prints:
      """
      .cucumbersortrc:3  unused regex
      """
    And the exit code is success
    And file "feature/one.feature" now has content:
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
