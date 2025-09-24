.DEFAULT_GOAL := help

cuke:  # runs the end-to-end tests
	@cargo test --test cuke

help:  # prints all available targets
	@grep -h -E '^[a-zA-Z_-]+:.*?# .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?# "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

test: cuke  # runs all tests
