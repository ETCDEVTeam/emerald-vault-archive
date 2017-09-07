#!/usr/bin/env bats

: ${EMERALD_CLI:=$HOME/.cargo/bin/emerald} # FIXME emerald or emerald-cli?


# Setup and teardown are called surrounding EACH @test.
setup() {
	EMERALD_BASE_PATH=`mktemp -d`
}

teardown() {
	rm -fr $EMERALD_BASE_PATH
}

@test "runs with valid command" {
	run $EMERALD_CLI --version
	[ "$status" -eq 0 ]
	[[ "$output" == *"v"* ]]
}
