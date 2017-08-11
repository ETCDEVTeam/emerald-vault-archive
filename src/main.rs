//! # CLI wrapper for `emerald-core`

#![cfg(feature = "cli")]

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate log;

extern crate docopt;
extern crate env_logger;
extern crate emerald_core as emerald;
extern crate regex;

use docopt::Docopt;
use emerald::keystore::KdfDepthLevel;
use env_logger::LogBuilder;
use log::{LogLevel, LogRecord};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::*;
use std::str::FromStr;

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

    if log_enabled!(LogLevel::Info) {
        info!("Starting Emerald Connector - v{}", emerald::version());
    }

    let chain = match args.flag_chain.parse::<String>() {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e.to_string());
            "mainnet".to_string()
        }
    };
    info!("Chain set to '{}'", chain);

    let sec_level_str: &str = &args.flag_security_level.parse::<String>().expect(
        "Expect to parse \
         security level",
    );

    let sec_level = match KdfDepthLevel::from_str(sec_level_str) {
        Ok(sec) => sec,
        Err(e) => {
            error!("{}", e.to_string());
            KdfDepthLevel::default()
        }
    };
    info!("Security level set to '{}'", sec_level);

    if args.cmd_server {

    }

}
