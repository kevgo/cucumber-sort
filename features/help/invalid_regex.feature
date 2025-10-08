Feature: format already ordered steps

  Scenario:
    Given file "cucumber-sort.json" with content:
      """
      step 1
      invalid (
      """
    When I run "cucumber-sort check"
    Then it prints the error:
      """
      cucumber-sort.json:1  invalid regular expression

      regex parse error:
          invalid (
                  ^
      error: unclosed group
      """
    And the exit code is failure
