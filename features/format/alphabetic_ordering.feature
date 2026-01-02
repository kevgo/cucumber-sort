Feature: alphabetically order similar steps

	@this
	Scenario: correct order but not sorted alphabetically
		Given file "cucumber-sort.json" with content:
			"""
			{
				"steps": [
					"file .*"
				]
			}
			"""
		And file "features/one.feature" with content:
			"""
      Feature: example

        Scenario: correct order but not sorted alphabetically
          Given file "gamma"
          And file "beta"
          And file "alpha"
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
          And file "gamma"
			"""
