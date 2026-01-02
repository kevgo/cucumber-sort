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
        "$schema": "https://raw.githubusercontent.com/kevgo/cucumber-sort/refs/heads/main/docs/schema.json",
        "include": [],
        "exclude": [],
        "record": false,
        "fail-fast": false,
        "keep-order": [],
        "steps": [],
        "unknown-steps": []
      }
      """
