//! # Command executor

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate emerald_core as emerald;

use std::path::PathBuf;
use std::str::FromStr;


#[derive(Debug, Deserialize)]
pub struct Args {
    flag_version: bool,
    flag_quiet: bool,
    flag_verbose: bool,
    flag_host: String,
    flag_port: String,
    flag_base_path: String,
    flag_security_level: String,
    flag_chain: String,
    cmd_server: bool,
    cmd_list: bool,
    cmd_new: bool,
    cmd_hide: bool,
    cmd_unhide: bool,
    cmd_update: bool,
    cmd_strip: bool,
    cmd_import: bool,
    cmd_export: bool,
    cmd_transaction: bool,
}

pub struct CmdExecutor;

impl CmdExecutor {
    ///
    pub fn run(args: Args) {
        if args.cmd_server {
            CmdExecutor::server()
        } else if args.cmd_list {
            CmdExecutor::list()
        } else if args.cmd_new {
            CmdExecutor::new_account()
        } else if args.cmd_hide {
            CmdExecutor::hide()
        } else if args.cmd_unhide {
            CmdExecutor::unhide()
        } else if args.cmd_update {
            CmdExecutor::update()
        } else if args.cmd_strip {
            CmdExecutor::strip()
        } else if args.cmd_import {
            CmdExecutor::import()
        } else if args.cmd_export {
            CmdExecutor::export()
        } else if args.cmd_transaction {
            CmdExecutor::sign_transaction()
        }
    }

    ///
    fn server(args: Args) {
        let addr = format!("{}:{}", args.flag_host, args.flag_port)
            .parse::<SocketAddr>()
            .expect("Expect to parse address");

        let base_path_str = args.flag_base_path.parse::<String>().expect(
            "Expect to parse base \
             path",
        );

        let base_path = if !base_path_str.is_empty() {
            Some(PathBuf::from(&base_path_str))
        } else {
            None
        };

        emerald::rpc::start(&addr, &chain, base_path, Some(sec_level));
    }

    ///
    fn list() {}

    ///
    fn new_account() {}

    ///
    fn hide() {}

    ///
    fn unhide() {}

    ///
    fn strip() {}

    ///
    fn export() {}

    ///
    fn import() {}

    ////
    fn update() {}

    ////
    fn sign_transaction() {}
}
