# dev tooling and versions
RUN_THAT_APP_VERSION = 0.18.0

clear:  # removes all temporary artifacts
	@rm -f tools/rta*
	@rm -rf tools/node_modules
	@rm -rf target

cuke: build  # runs the end-to-end tests
	@cargo test --quiet --locked --test cuke

cukethis: build  # runs only end-to-end tests with a @this tag
	@cargo test --test cuke --quiet --locked -- -t @this

fix: tools/rta@${RUN_THAT_APP_VERSION}  # auto-corrects issues
	tools/rta dprint fmt
	cargo +nightly fmt
	cargo clippy --all-targets --all-features --quiet -- --deny=warnings
	cargo +nightly fix --allow-dirty --quiet
	tools/rta ghokin fmt replace features/

lint: tools/node_modules tools/rta@${RUN_THAT_APP_VERSION}  # checks formatting
	tools/rta dprint check
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo +nightly fmt -- --check
	git diff --check
	tools/rta actionlint
	cargo machete
	tools/rta node tools/node_modules/.bin/gherkin-lint

setup: setup-ci  # install development dependencies on this computer
	cargo install cargo-edit cargo-upgrades --locked

test: build unit fix lint cuke   # runs all tests

unit:  # runs the unit tests
	cargo test --locked

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

build:
	@cargo build --quiet

help:  # prints all available targets
	@grep -h -E '^[a-zA-Z_-]+:.*?# .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?# "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup-ci:
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

tools/rta@${RUN_THAT_APP_VERSION}:
	@rm -f tools/rta* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta

tools/node_modules: tools/package-lock.json tools/rta@${RUN_THAT_APP_VERSION}
	@echo "Installing Node based tools"
	cd tools && ./rta npm ci
	@touch tools/node_modules  # update timestamp of the node_modules folder so that Make doesn't re-install it on every command

.SILENT:
.DEFAULT_GOAL := help
