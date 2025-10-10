Feature: check unordered steps

  Scenario: dependent step
    Given file "cucumber-sort.json" with content:
      """
      {
        "steps": [
          "a repo",
          ["the branches", "the commits"],
          "a file",
          "another thing"
        ]
      }
      """
    And file "features/one.feature" with content:
      """
      Feature: example
      
        Scenario: steps out of order
          Given a repo
          And a file
          And the branches
            | branch-1 |
            | branch-2 |
          And the commits
            | commit 1 |
            | commit 2 |
          And another thing
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      features/one.feature:4  expected Then step 1 but found Then step 2
      features/one.feature:5  expected And step 2 but found And step 1
      features/two.feature:4  expected Then step 1 but found Then step 2
      features/two.feature:5  expected And step 2 but found And step 1
      """
    And the exit code is failure
