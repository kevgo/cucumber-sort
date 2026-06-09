RUN_THAT_APP_VERSION = 0.37.0

RTA      = tools/rta@$(RUN_THAT_APP_VERSION)
ACTIONLINT = $(RTA) actionlint
CUCUMBER_SORT = $(RTA) cucumber-sort
DPRINT = $(RTA) dprint
GHERKIN_LINT = $(RTA) node node_modules/.bin/gherkin-lint
GHOKIN   = $(RTA) ghokin
NODE = $(RTA) node
NPM = $(RTA) npm
TEXTRUNNER = $(RTA) node node_modules/.bin/text-runner


clear:  # removes all temporary artifacts
	rm -f tools/rta*
	rm -rf node_modules
	rm -rf target

contest: ${RTA}  # starts the contest server
	${RTA} contest

cuke: build  # runs the end-to-end tests
	cargo test --quiet --locked --test cuke

cukethis: build  # runs only end-to-end tests with a @this tag
	cargo test --test cuke --quiet --locked -- -t @this

doc: build node_modules
	${TEXTRUNNER} . --format=dot

fix: tools/rta@${RUN_THAT_APP_VERSION}  # auto-corrects issues
	${DPRINT} fmt
	cargo +nightly fmt
	cargo clippy --all-targets --all-features --quiet -- --deny=warnings
	cargo +nightly fix --allow-dirty --quiet
	${GHOKIN} fmt replace features/

install:  # installs the binary on the local machine
	cargo install --locked --path .

lint: node_modules ${RTA}  # checks formatting
	${DPRINT} check
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo +nightly fmt -- --check
	git diff --check
	${ACTIONLINT}
	cargo machete
	${GHERKIN_LINT}

ps: fix test  # pitstop, run during development
	cargo run --quiet -- check

setup: setup-ci  # install development dependencies on this computer
	cargo install cargo-edit cargo-upgrades --locked

setup-ci: node_modules
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

test: build unit lint cuke doc   # runs all tests

unit:  # runs the unit tests
	@cargo test --locked

update: ${RTA}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	$(RTA) --update
	$(NPM) update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

build:
	cargo build --quiet

help:  # prints all available targets
	grep -h -E '^[a-zA-Z_-]+:.*?# .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?# "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

${RTA}:
	@rm -f tools/rta*
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RUN_THAT_APP_VERSION} --name rta@${RUN_THAT_APP_VERSION})

node_modules: package-lock.json ${RTA}
	${NPM} ci
	touch node_modules  # update timestamp of the node_modules folder so that Make doesn't re-install it on every command

.SILENT:
.DEFAULT_GOAL := help
