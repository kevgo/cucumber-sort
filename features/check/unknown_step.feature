Feature: check unknown steps

  Background:
    Given file ".cucumber-sort-order" with content:
      """
      step 1
      """
    And file "features/one.feature" with content:
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

        Scenario: contains a different unknown step
          Given another unknown step
      """

  Scenario: run without recording
    When I run "cucumber-sort check"
    Then it prints:
      """
      features/one.feature:5  unknown step: file "foo.feature" with content:
      features/one.feature:11  unknown step: file "foo.feature" with content:
      features/one.feature:17  unknown step: another unknown step
      """
    And the exit code is failure

  Scenario: run with recording
    When I run "cucumber-sort check --record"
    Then it prints:
      """
      features/one.feature:5  unknown step: file "foo.feature" with content:
      features/one.feature:11  unknown step: file "foo.feature" with content:
      features/one.feature:17  unknown step: another unknown step
      """
    And the exit code is failure
    And file ".cucumber-sort-order" now has content:
      """
      step 1

      # UNKNOWN STEPS
      ^another unknown step$
      ^file ".*" with content:$
      """

  Scenario: run with recording and existing marker
    Given file ".cucumber-sort-order" with content:
      """
      step 1

      # UNKNOWN STEPS
      ^another unknown step$
      ^file .* with content:$
      """
    When I run "cucumber-sort check --record"
    Then it prints:
      """
      features/one.feature:5  unknown step: file "foo.feature" with content:
      features/one.feature:11  unknown step: file "foo.feature" with content:
      features/one.feature:17  unknown step: another unknown step
      """
    And the exit code is failure
    And file ".cucumber-sort-order" now has content:
      """
      step 1

      # UNKNOWN STEPS
      ^another unknown step$
      ^file ".*" with content:$
      """
