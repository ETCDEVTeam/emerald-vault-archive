#!/usr/bin/env bats

: ${EMERALD_CLI:=$HOME/.cargo/bin/emerald} # FIXME emerald or emerald-cli?


# Setup and teardown are called surrounding EACH @test.
setup() {
	export EMERALD_BASE_PATH=`mktemp -d`
}

teardown() {
	rm -rf $EMERALD_BASE_PATH
    unset EMERALD_BASE_PATH
}

@test "[meta] succeeds: set env var and tmp dir EMERALD_BASE_PATH" {
    run echo "$EMERALD_BASE_PATH"
    [ "$status" -eq 0 ]
	[ -d $EMERALD_BASE_PATH ]
    [ "$output" != "" ]
}

@test "succeeds: --version" {
	run $EMERALD_CLI --version
	[ "$status" -eq 0 ]
	[[ "$output" == *"v"* ]]
}

@test "succeeds: --help" {
    run $EMERALD_CLI --help
    [ "$status" -eq 0 ]
    [[ "$output" == *"Emerald"* ]]
    [[ "$output" == *"Usage"* ]]
    [[ "$output" == *"Options"* ]]
}

@test "succeeds: new --chain=testnet [empty options]" {
    run $EMERALD_CLI new \
        --chain=testnet \
        <<< $'foo\n'
    [ "$status" -eq 0 ]
    [[ "$output" == *"Created new account"* ]]
}

@test "succeeds: new --chain=mainnet --security=high --name='Test account' --description='Some description'" {
    run $EMERALD_CLI new \
        --chain=mainnet \
        --security-level=high \
        --name="Test account" \
        --description="Some description" \
        <<< $'foo\n'
    [ "$status" -eq 0 ]
    [[ "$output" == *"Created new account"* ]]
}
