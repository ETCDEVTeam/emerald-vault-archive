```shell
                                                          __       __                  __    _
                ___    ____ ___   ___    _____  ____ _   / /  ____/ /         _____   / /   (_)
               / _ \  / __ `__ \ / _ \  / ___/ / __ `/  / /  / __  /  ______ / ___/  / /   / / 
              /  __/ / / / / / //  __/ / /    / /_/ /  / /  / /_/ /  /_____// /__   / /   / /  
              \___/ /_/ /_/ /_/ \___/ /_/     \__,_/  /_/   \__,_/          \___/  /_/   /_/   
                                                                                 
```
<p align="center">
  <p align="center">
    <a href="https://travis-ci.org/ETCDEVTeam/emerald-cli"><img alt="Travis" src="https://travis-ci.org/ETCDEVTeam/emerald-cli.svg?branch=master"></a>
    <a href="https://circleci.com/gh/etcdevteam/emerald-cli"><img alt="CircleCI" src="https://circleci.com/gh/ETCDEVTeam/emerald-cli/tree/master.svg?style=shield"></a>
    <a href="https://ci.appveyor.com/project/etcdevteam/emerald-cli"><img alt="AppVeyor" src="https://ci.appveyor.com/api/projects/status/9h3kobw811vmynk7?svg=true"></a>
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

```shell
$ emerald --help

emerald
Command-line interface for Emerald platform

USAGE:
    emerald [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Display version

OPTIONS:
    -p, --base-path <base-path>    Set path for chain storage
    -c, --chain <chain>            Sets a chain name [default: mainnet]

SUBCOMMANDS:
    account        Account related commands
    balance        Request account's balance from ethereum node through RPC
    help           Prints this message or the help of the given subcommand(s)
    mnemonic       Create mnemonic phrase according to BIP39 spec
    server         Start local RPC server
    transaction    Transaction related commands

```

For detailed documentation see [https://docs.etcdevteam.com/html/emerald-cli](https://docs.etcdevteam.com/html/emerald-cli)

## Installing Emerald CLI

### Download stable binary

Binaries for all platforms are currently published at https://github.com/ETCDEVTeam/emerald-cli/releases

### :beers: Install with Homebrew (OSX only)

Install latest stable binary.

```
$ brew install ethereumproject/classic/emerald-cli
```

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

<!-- ## Demo --> 

<!-- <a href="https://asciinema.org/a/WbivFQXwm5lUXenNsTvzfQxRY?speed=2" target="_blank"> -->
  <!-- <img src="https://asciinema.org/a/WbivFQXwm5lUXenNsTvzfQxRY.png" /> -->
<!-- </a> -->

## License

Apache 2.0

#### Documentation:
- [Installation](docs/install.md)
- [CLI Usage](docs/cli.md)

