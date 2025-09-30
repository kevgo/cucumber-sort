Feature: check ordered steps

  Scenario:
    Given file ".cucumber-sort-rc" with content:
      """
      file .* with content:
      step 2
      step 3
      file .* now has content:
      step 5
      """
    And file "feature/one.feature" with content:
      """
      Feature: example

        Background:
          Given file "foo" with content:
            '''
            bar
            '''
          And step 2
          When step 3

        Scenario: result
          Then file "foo" now has content:
            '''
            bar
            '''
          And step 5
      """
    When I run "cucumber-sort check"
    Then it prints nothing
    And the exit code is success
