Feature: list unused steps while checking

  Scenario: unordered step in a scenario
    Given file ".cucumbersortrc" with content:
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
      .cucumbersortrc:2  unused regex: file .* now has content:
      """
    And the exit code is success
