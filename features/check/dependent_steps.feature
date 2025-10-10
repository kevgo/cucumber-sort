Feature: check unordered steps

  Scenario: dependent steps in given order
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
      features/one.feature:5  expected And the branches but found And a file
      features/one.feature:6  expected | branch-1 | but found And the branches
      features/one.feature:7  expected | branch-2 | but found | branch-1 |
      features/one.feature:8  expected And the commits but found | branch-2 |
      features/one.feature:9  expected | commit 1 | but found And the commits
      features/one.feature:10  expected | commit 2 | but found | commit 1 |
      features/one.feature:11  expected And a file but found | commit 2 |
      """
    And the exit code is failure

  @this
  Scenario: dependent steps in opposite order
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
          And the commits
            | commit 1 |
            | commit 2 |
          And a file
          And the branches
            | branch-1 |
            | branch-2 |
          And another thing
      """
    When I run "cucumber-sort check"
    Then it prints:
      """
      features/one.feature:5  expected And the branches but found And the commits
      features/one.feature:6  expected | branch-1 | but found | commit 1 |
      features/one.feature:7  expected | branch-2 | but found | commit 2 |
      features/one.feature:8  expected And the commits but found And a file
      features/one.feature:9  expected | commit 1 | but found And the branches
      features/one.feature:10  expected | commit 2 | but found | branch-1 |
      features/one.feature:11  expected And a file but found | branch-2 |
      """
    And the exit code is failure
