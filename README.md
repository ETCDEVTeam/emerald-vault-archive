```shell                                                  __       __                  __    _ 
          ___    ____ ___   ___    _____  ____ _   / /  ____/ /         _____   / /   (_)
         / _ \  / __ `__ \ / _ \  / ___/ / __ `/  / /  / __  /  ______ / ___/  / /   / / 
        /  __/ / / / / / //  __/ / /    / /_/ /  / /  / /_/ /  /_____// /__   / /   / /  
        \___/ /_/ /_/ /_/ \___/ /_/     \__,_/  /_/   \__,_/          \___/  /_/   /_/   
                                                                                 
```
<p align="center">
  <p align="center">
    <a href="https://circleci.com/gh/etcdevteam/emerald-cli"><img alt="CircleCI" src="https://circleci.com/gh/ETCDEVTeam/emerald-cli/tree/master.svg?style=shield"></a>
    <a href="https://ci.appveyor.com/project/splix/emerald-cli-759r3"><img alt="AppVeyor" src="https://ci.appveyor.com/api/projects/status/9h3kobw811vmynk7?svg=true"></a>
    <a href="https://crates.io/crates/emerald-cli"><img alt="crates.io" src="https://img.shields.io/crates/v/emerald-cli.svg?style=flat-square"></a>
    <a href="LICENSE"><img alt="Software License" src="https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square&maxAge=2592000"></a>
  </p>
</p>

## About

`Emerald Platform` is a set of tools to build and integrate other apps with Ethereum ETC blockchain.
`Emerald CLI(Command Line Interface)`  is a tool to access Ethereum ETC from command line. It connects to an external node (_"Upstream"_) and
allows to read information from blockchain and send new transactions. At the later case it provides functionality
to sign transaction by provided Private Key. The tool integrates Emerald Vault with designed to generate, import and/or
store Ethereum Private Keys

It's compatible with both Ethereum ETC and ETH


## Usage

```
emerald --help

Emerald offline wallet command line interface.

Usage:
  emerald server    [--chain=<chain>] [--port=<port>] [--host=<host>] [--base-path=<path>] [-v | --verbose] [-q | --quite]
  emerald mnemonic
  emerald new       [--chain=<chain>]  ([[--security-level=<level>] [--name=<name>] [--description=<description>]] | --raw <key>)
  emerald list      [--chain=<chain>]  [--show-hidden]
  emerald hide      [--chain=<chain>]  <address>
  emerald unhide    [--chain=<chain>]  ([-a | --all] | <address>)
  emerald strip     [--chain=<chain>] <address>
  emerald import    [--chain=<chain>]  [-a | --all] [-f | --force] <path>
  emerald export    [--chain=<chain>]  ([-a | --all] | <address>) <path>
  emerald update    [--chain=<chain>]  <address> [--name=<name>] [--description=<description>]
  emerald transaction   [--chain=<chain>] <from> <to> <value> [--gas=<gas>] [--gas-price=<price>] [--data=<data>] (--nonce=<nonce> | --upstream=<upstream>)
  emerald balance   <address> [--upstream=<upstream>]
  emerald -V | --version
  emerald -h | --help

Options:
  -a, --all                                   Apply action to all accounts
  -c, --chain=<mainnet|testnet>               Chain name
  -f, --force                                 Override existing keystore file
  -h, --help                                  Show this message
  -V, --version                               Show current version
  -r, --raw                                   Create Keyfile directly from a private key
  -q, --quiet                                 Only errors printed to the output
  -v, --verbose                               Verbose output
      --data=<data>                           Optional data included in a transaction
      --name=<name>                           Account name
      --description=<description>             Account description
      --host=<host>                           Listen host [default: 127.0.0.1]
      --port=<port>                           Listen port [default: 1920]
      --base-path=<path>                      Base directory path, if omitted default os-specific value will be used:
                                                  + Mac OS X: ~/Library/Emerald
                                                  + Linux: ~/.emerald
                                                  + Windows: %USERDIR%\.emerald
      --security-level=<normal|high|ultra>    Level of security for cryptographic operations [default: ultra]
      --show-hidden                           Include hidden keyfiles
      --upstream=<upstream>                   URL to ethereum node [default: 127.0.0.1:8545]
      --gas=<gas>                             Gas limit for transaction, hex-encoded value  in `wei`
      --gas-price=<price>                     Gas price for transaction, hex-encoded value  in `wei`
      --nonce=<nonce>                         Transaction count of sender

```

For detailed documentation see [https://docs.etcdevteam.com/html/emerald-cli](https://docs.etcdevteam.com/html/emerald-cli)

## Installing Emerald CLI

### Download stable binary

Binaries for all platforms are currently published at https://github.com/ETCDEVTeam/emerald-cli/releases

### Download development build


Development builds are usually unstable and may contain critical issues that can lead to loss of funds. Use it on your risk


ETCDEV has a dedicated website for all build artifacts, which are published on each new commit into `master` branch.
To download a latest development build, please open https://builds.etcdevteam.com and choose _Emerald CLI_ tab


### Build from sources

#### Requirements

Install Rust from https://www.rust-lang.org/en-US/install.html

  
Unix one-liner:
```
curl https://sh.rustup.rs -sSf | sh
```
  
On Windows, Rust additionally requires the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire
the build tools is by installing Microsoft Visual C++ Build Tools 2017 which provides just the Visual C++ build tools.
  
#### Compile

```
git clone https://github.com/etcdevteam/emerald-cli.git
cd emerald-cli
cargo build --release
cd target\debug
```

## Links

- Documentation: https://docs.etcdevteam.com/html/emerald-cli
- Issues: https://github.com/ETCDEVTeam/emerald-cli/issues
- Development binaries: http://builds.etcdevteam.com/

## Demo

<a href="https://asciinema.org/a/WbivFQXwm5lUXenNsTvzfQxRY?speed=2" target="_blank">
  <img src="https://asciinema.org/a/WbivFQXwm5lUXenNsTvzfQxRY.png" />
</a>

## License

Apache 2.0

