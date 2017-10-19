# CLI API

## Usage

Usage help description is based on [DOCOPT](http://docopt.org/) link:{rootdir}/emerald-cli/usage.txt[`usage.txt`].

use `-h | --help` for help menu

## Environment variables

Environment variables allow you to redefine the default settings:

* `EMERALD_HOST` - listen host
* `EMERALD_PORT` - listen port
* `EMERALD_CHAIN` - chain name (`mainnet` | `testnet`), has a higher priority relative to `EMERALD_CHAIN_ID`
* `EMERALD_CHAIN_ID` - chain id number, has a lower priority relative to `EMERALD_CHAIN`
* `EMERALD_GAS` - maximum gas limit to use by transaction
* `EMERALD_GAS_PRICE` - gas cost to use by transaction (in Gwei)
* `EMERALD_SECURITY_LEVEL` - security level (`normal` | `high` | `ultra`)
* `EMERALD_NODE` - url to node. Used for sign and send of transactions

## HOWTO

### How to run [JSON-RPC](http://github.com/ethereumproject/emerald-rs/blob/master/docs/api.adoc) endpoint

```
    emerald server --host=127.0.0.1 --port=1920
```

### How to show all available accounts

```
    emerald list --chain=testnet
```

### How to exclude some accounts from the showing list

```
    emerald hide --chain=testnet 0x0e7c045110b8dbf29765047380898919c5cb56f4
```

To undo in the future:

```
    emerald unhide --chain=testnet --all
```

### How to create new account

```
    emerald new --chain=testnet \
        --security-level=high \
        --name="Test account" \
        --description="Some description" \
        < echo "secret passphrase"
```

### How to show private key

```
    emerald strip --chain=testnet 0x0e7c045110b8dbf29765047380898919c5cb56f4 < echo "secret passphrase"
```

### How to change `passphrase`

```
    emerald strip --chain=testnet 0x0e7c045110b8dbf29765047380898919c5cb56f4 < echo "old passphrase" \
    emerald new --chain=testnet --raw < echo "new passphrase"
```

### How to change account name

```
    emerald update --chain=testnet \
        0x0e7c045110b8dbf29765047380898919c5cb56f4 \
        --name="New name" \
        --description="A new description"
```

### How to export & import all accounts
Import content of whole folder:
```
    emerald import --chain=testnet --all <path_to_files>
```
or single keyfile:
```
    emerald import --chain=testnet <path_to_file>
```
If keyfile already exist in a storage, import will be ignore.

To override existing Keyfile, use `-f | --force` option:
```
    emerald import --chain=testnet --force <path_to_file>
```


Export all keyfiles into directory:
```
    emerald export --chain=testnet --all <path_to_export_dir>
```
or single keyfile for selected <address>:
```
    emerald export --chain=testnet <address> <path_to_export_dir>
```

### How to get balance for address
If using client that running on a local host (127.0.0.1:8545):
```
emerald balance 0x0e7c045110b8dbf29765047380898919c5cb56f4
```
or connecting to some specific host & port:
```
emerald balance 0x0e7c045110b8dbf29765047380898919c5cb56f4 --upstream=8.8.8.8:8545
```


### How to sign transaction

Offline:
```
    EMERALD_GAS_COST=21 \
    emerald transaction \
        0x0e7c045110b8dbf29765047380898919c5cb56f4 \
        0x0e7c045110b8dbf29765047380898919c5cb56f4 \
        0x1000 \
        --gas=0x2100 \
        --nonce=0x10001 \
        < echo "secret passphrase"
```

or sent transaction for execution through remote node:
```
    EMERALD_GAS_COST=21 \
    emerald transaction \
        0x0e7c045110b8dbf29765047380898919c5cb56f4 \
        0x0e7c045110b8dbf29765047380898919c5cb56f4 \
        0x1000 \
        --gas=0x2100 \
        --upstream=127.0.0.1:8545 \
        < echo "secret passphrase"
```