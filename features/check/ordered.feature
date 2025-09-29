Feature: check ordered steps

  @this
  Scenario:
    Given file ".cucumbersortrc" with content:
      """
      file .* with content:
      step 2
      step 3
      file .* now has content:
      step 5
      """
    And file "feature/one.feature" with content:
      """
      Feature: example
      
        Background:
          Given file "foo" with content:
            _"_"_"
            bar
            _"_"_"
          And step 2
          When step 3
      
        Scenario: result
          Then file "foo" now has content:
            _"_"_"
            bar
            _"_"_"
          And step 5
      """
    When I run "cucumber-sort check"
    Then it prints nothing
    And the exit code is success
