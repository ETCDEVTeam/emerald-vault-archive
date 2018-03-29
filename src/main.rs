//! # CLI wrapper for `emerald-rs`

#![cfg(feature = "cli")]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate docopt;
extern crate emerald_rs as emerald;
extern crate env_logger;
extern crate hex;
extern crate hyper;
extern crate jsonrpc_core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate rpassword;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
#[macro_use]
extern crate clap;

mod cmd;
mod rpc;

use cmd::CmdExecutor;
use docopt::Docopt;
use env_logger::LogBuilder;
use log::LogRecord;
use std::env;
use std::process::*;
use clap::{App, SubCommand};

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

/// Get the current Emerald version.
pub fn version() -> &'static str {
    VERSION.unwrap_or("unknown")
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches
        .subcommand_matches("transaction")
        .and_then(|m| m.subcommand_matches("send"))
    {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    //    let flags = vec![(args.flag_verbose, "trace"), (args.flag_quiet, "error")];
    //
    //    let (_, verbosity) = *flags
    //        .into_iter()
    //        .filter(|&(flag, _)| flag)
    //        .collect::<Vec<(bool, &str)>>()
    //        .last()
    //        .unwrap_or(&(true, "emerald=info"));
    //
    //    env::set_var("RUST_LOG", verbosity);
    //
    //    let mut log_builder = LogBuilder::new();
    //    if env::var("RUST_LOG").is_ok() {
    //        log_builder.parse(&env::var("RUST_LOG").unwrap());
    //    }
    //    log_builder.format(|record: &LogRecord| format!("[{}]\t{}", record.level(), record.args()));
    //    log_builder.init().expect("Expect to initialize logger");

    if matches.is_present("version") {
        println!("v{}", version());
        exit(0);
    }

    let cmd = match CmdExecutor::new(&matches) {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e.to_string());
            exit(1)
        }
    };

    match cmd.run() {
        Ok(_) => exit(0),
        Err(e) => {
            error!("{}", e.to_string());
            exit(1);
        }
    }
}
