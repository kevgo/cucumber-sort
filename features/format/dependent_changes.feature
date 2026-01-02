Feature: format dependent steps

  Scenario: dependent step
    Given file "cucumber-sort.json" with content:
      """
      {
        "steps": [
          "a repo",
          [
            "the branches",
            "the commits"
          ],
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
          And the branches
            | branch-3 |
            | branch-4 |
          And the commits
            | commit 3 |
            | commit 4 |
      """
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
    And file "features/one.feature" now has content:
      """
      Feature: example

        Scenario: steps out of order
          Given a repo
          And the branches
            | branch-1 |
            | branch-2 |
          And the commits
            | commit 1 |
            | commit 2 |
          And the branches
            | branch-3 |
            | branch-4 |
          And the commits
            | commit 3 |
            | commit 4 |
          And a file
          And another thing
      """

  Scenario: dependent steps in opposite order
    Given file "cucumber-sort.json" with content:
      """
      {
        "steps": [
          "a repo",
          [
            "the branches",
            "the commits"
          ],
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
    When I run "cucumber-sort format"
    Then it prints nothing
    And the exit code is success
    And file "features/one.feature" now has content:
      """
      Feature: example

        Scenario: steps out of order
          Given a repo
          And the commits
            | commit 1 |
            | commit 2 |
          And the branches
            | branch-1 |
            | branch-2 |
          And a file
          And another thing
      """
