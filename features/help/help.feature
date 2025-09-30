Feature: requesting help

  Scenario:
    When I run "cucumber-sort help"
    Then it prints:
      """
      Sorts steps in Cucumber files

      Usage: cucumber-sort <COMMAND>

      Commands:
        check   Check if Cucumber files are properly sorted
        format  Format Cucumber files by sorting them
        init    Create the configuration files
        help    Print this message or the help of the given subcommand(s)

      Options:
        -h, --help  Print help
      """
    And the exit code is success
