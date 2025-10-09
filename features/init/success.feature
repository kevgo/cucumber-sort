Feature: creating the config files

  Scenario:
    When I run "cucumber-sort init"
    Then it prints:
      """
      config file created: cucumber-sort.json
      """
    And file "cucumber-sort.json" now has content:
      """
      {
        "include": [],
        "exclude": [],
        "record": false,
        "fail-fast": false,
        "steps": [],
        "unknown-steps": []
      }
      """
