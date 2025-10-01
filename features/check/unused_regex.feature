Feature: list unused steps while checking

  Scenario: doesn't use "file .* now has content" regex
    Given file ".cucumber-sort-rc" with content:
      """
      file .* with content:
      file .* now has content:
      """
    And file "feature/one.feature" with content:
      """
      Feature: example
      
        Scenario: test
          Given file "foo" with content:
            '''
            bar
            '''
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      .cucumber-sort-rc:2  unused regex: file .* now has content:
      """
    And the exit code is failure
