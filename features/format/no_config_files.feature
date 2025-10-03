Feature: run without config file

  Background:
    Given file "features/one.feature" with content:
      """
      Feature: example
      
        Scenario: test
          Given file "foo" with content:
            '''
            bar
            '''
          And step 2
      """

  Scenario: format
    When I run "cucumber-sort format"
    Then it prints:
      """
      features/one.feature:4  unknown step: file "foo" with content:
      features/one.feature:8  unknown step: step 2
      """
    And the exit code is failure
    And file contents haven't changed

  Scenario: format and record
    When I run "cucumber-sort format --record"
    Then it prints:
      """
      features/one.feature:4  unknown step: file "foo" with content:
      features/one.feature:8  unknown step: step 2
      """
    And the exit code is failure
    And file ".cucumber-sort-order" now has content:
      """
      # UNKNOWN STEPS
      ^file ".*" with content:$
      ^step 2$
      """
    And file contents haven't changed
