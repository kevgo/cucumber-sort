Feature: docstring with empty line

  Scenario: docstring with empty line
    Given file ".cucumber-sort-order" with content:
      """
      step 1
      step 2
      file .* with content:
      """
    And file "features/one.feature" with content:
      """
      Feature: example

        Scenario: test
          Given step 1
          And file "foo" with content:
            '''
            Feature: example

              Scenario: steps out of order
                When step 1
                And step 2
            '''
          And step 2
      """
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
    And file "features/one.feature" now has content:
      """
      Feature: example

        Scenario: test
          Given step 1
          And step 2
          And file "foo" with content:
            '''
            Feature: example

              Scenario: steps out of order
                When step 1
                And step 2
            '''
      """
