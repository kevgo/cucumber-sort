Feature: enforce alphabetical ordering of similar steps

	# @this
	Scenario:
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
		When I run "cucumber-sort check"
		Then it prints:
			"""
			features/one.feature:4  expected Given file "alpha" but found Given file "gamma"
			features/one.feature:6  expected And file "gamma" but found And file "alpha"
			"""
		And the exit code is failure
