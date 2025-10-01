Feature: check unknown steps

  Background: "step 3" is not defined in the config file
    Given file ".cucumber-sort-rc" with content:
      """
      step 1
      """
    And file "feature/one.feature" with content:
      """
      Feature: example
      
        Background: contains an unknown step
          Given step 1
          And file "foo.feature" with content:
            '''
            bar
            '''
      
        Scenario: contains the same unknown step
          Given file "foo.feature" with content:
            '''
            bar
            '''
      
        Scenario: contains another unknown step
          Given another unknown step
      """

  Scenario: run without recording
    When I run "cucumber-sort check"
    Then it prints:
      """
      feature/one.feature:5  unknown step: file .* with content:
      feature/one.feature:11  unknown step: file .* with content:
      feature/one.feature:17  unknown step: another unknown step
      """
    And the exit code is failure

  @this
  Scenario: run with recording
    When I run "cucumber-sort check --record"
    Then it prints:
      """
      feature/one.feature:5  unknown step: file .* with content:
      feature/one.feature:11  unknown step: file .* with content:
      feature/one.feature:17  unknown step: another unknown step
      """
    And the exit code is failure
    And file ".cucumber-sort-rc" now has content:
      """
      step 1
      
      # UNKNOWN STEPS
      file .* with content:
      """
