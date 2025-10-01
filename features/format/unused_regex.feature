Feature: formatting in the presence of an unused regex

  Background:
    Given file ".cucumber-sort-rc" with content:
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

  Scenario: without record
    When I run "cucumber-sort format"
    Then it prints:
      """
      .cucumber-sort-rc:3  unused regex: file .* now has content:
      """
    And the exit code is failure
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
    And file ".cucumber-sort-rc" hasn't changed

  Scenario: with record
    When I run "cucumber-sort format --record"
    Then it prints:
      """
      .cucumber-sort-rc:3  unused regex: file .* now has content:
      """
    And the exit code is failure
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
    And file ".cucumber-sort-rc" hasn't changed
