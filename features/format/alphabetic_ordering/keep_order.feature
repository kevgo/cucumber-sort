Feature: keep order of some steps

  @this
  Scenario: correct order but not sorted alphabetically
    Given file "cucumber-sort.json" with content:
      """
      {
        "steps": [
          "file .*",
          "I ran .*"
        ],
        "keep-order": [
          "I ran .*"
        ]
      }
      """
    And file "features/one.feature" with content:
      """
      Feature: example

        Scenario: correct order but not sorted alphabetically
          Given file "beta"
          And file "alpha"
          And I ran "git branch -d beta"
          And I ran "git branch -d alpha"
      """
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
    And file "features/one.feature" now has content:
      """
      Feature: example

        Scenario: correct order but not sorted alphabetically
          Given file "alpha"
          And file "beta"
          And I ran "git branch -d beta"
          And I ran "git branch -d alpha"
      """
