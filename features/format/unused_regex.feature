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
      .cucumbersortrc:3  unused regex: file .* now has content:
      """
    And the exit code is success
    And file "feature/one.feature" now has content:
      """
      Feature: example
      
        Scenario: test
          Given file "foo" with content:
            '''
            bar
            '''
          And step 1
      """
