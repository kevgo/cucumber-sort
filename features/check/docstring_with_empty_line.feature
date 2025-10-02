Feature: docstring with empty line

  Scenario: docstring with empty line
    Given file ".cucumber-sort-order" with content:
      """
      file .* with content:
      """
    And file "features/one.feature" with content:
      """
      Feature: example

        Scenario: test
          Given file "foo" with content:
            '''
            Feature: example

              Scenario: steps out of order
                When step 1
                And step 2
            '''
      """
    When I run "cucumber-sort check"
    Then it prints nothing
    And the exit code is success
