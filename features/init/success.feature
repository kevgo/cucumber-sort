Feature: creating the config files

  Scenario:
    When I run "cucumber-sort init"
    Then it prints:
      """
      config files created
      """
    And file ".cucumber-sort-rc" now has content:
      """
      # More info at https://github.com/kevgo/cucumber-sort
      #
      # This file lists Gherkin steps in the desired order
      # without Given/When/Then, using regular expressions.

      # step 1
      # step 2
      """
    And file ".cucumber-sort-ignore" now has content:
      """
      # More info at https://github.com/kevgo/cucumber-sort
      #
      # This file lists files that cucumber-sort should ignore,
      # using glob expressions.

      # features/foo.feature
      """
