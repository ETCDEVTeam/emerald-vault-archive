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

@test "succeeds: list" {
    run $EMERALD_CLI new \
        --chain=testnet \
        <<< $'foo\n'
    [ "$status" -eq 0 ]
    [[ "$output" == *"Created new account"* ]]

    address=$(echo "$output" | perl -lane 'print $F[-1]')
    [[ "$address" != "" ]]
    [[ "$address" == *"0x"* ]]

    run $EMERALD_CLI list \
        --chain=testnet
    echo "$output" # prints in case fails
    echo "$address"

    [ "$status" -eq 0 ]
    [[ "$output" == *"Total: 1"* ]]
    # FIXME Why not work? Possibly because has newline in it.
    # [[ "$output" == *"Account: $address, name: , description:"* ]]
    unset address
}

@test "succeeds: update" {
    run $EMERALD_CLI new \
        --chain=testnet \
        <<< $'foo\n'
    [ "$status" -eq 0 ]
    [[ "$output" == *"Created new account"* ]]

    address=$(echo "$output" | perl -lane 'print $F[-1]')
    [[ "$address" != "" ]]
    [[ "$address" == *"0x"* ]]

    # FIXME Address probably with newline is still messing this up.
    # I gotta learn me some awk.
    run $EMERALD_CLI update \
        --chain=testnet \
        "$address" \
        --name="NewName" \
        --description="NewDescription"
        # <<< $'foo\n'

    echo "$output"
    [ "$status" -eq 0 ]
}

# perl -lane 'print $F[-1]'
