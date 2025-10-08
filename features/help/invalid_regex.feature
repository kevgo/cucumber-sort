Feature: format already ordered steps

  Scenario:
    Given file "cucumber-sort.json" with content:
      """
      {
        "steps": [
          "step 1",
          "invalid ("
        ]
      }
      """
    When I run "cucumber-sort check"
    Then it prints the error:
      """
      cucumber-sort.json:1  invalid regular expression
      
      Invalid regex 'invalid (': regex parse error:
          invalid (
                  ^
      error: unclosed group
      """
    And the exit code is failure
