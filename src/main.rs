//! # CLI wrapper for `emerald-rs`

#![cfg(feature = "cli")]

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
extern crate env_logger;
extern crate emerald_rs as emerald;
extern crate regex;
extern crate rustc_serialize;
extern crate serde_json;
extern crate jsonrpc_core;
extern crate hyper;
extern crate reqwest;
extern crate hex;


mod ctrl;
mod rpc;

use ctrl::{Args, CmdExecutor};
use docopt::Docopt;
use env_logger::LogBuilder;
use log::LogRecord;
use std::env;
use std::process::*;

const USAGE: &'static str = include_str!("../usage.txt");


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let flags = vec![(args.flag_verbose, "trace"), (args.flag_quiet, "error")];

    let (_, verbosity) = *flags
        .into_iter()
        .filter(|&(flag, _)| flag)
        .collect::<Vec<(bool, &str)>>()
        .last()
        .unwrap_or(&(true, "emerald=info"));

    env::set_var("RUST_LOG", verbosity);

    let mut log_builder = LogBuilder::new();
    if env::var("RUST_LOG").is_ok() {
        log_builder.parse(&env::var("RUST_LOG").unwrap());
    }
    log_builder.format(|record: &LogRecord| {
        format!("[{}]\t{}", record.level(), record.args())
    });
    log_builder.init().expect("Expect to initialize logger");

    if args.flag_version {
        println!("v{}", emerald::version());
        exit(0);
    }

    let cmd = match CmdExecutor::new(&args) {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e.to_string());
            exit(1);
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
