//! # Command executor

mod error;

use super::KdfDepthLevel;
use super::emerald;
use super::log::LogLevel;
use self::error::Error;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::str::FromStr;


#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_version: bool,
    pub flag_quiet: bool,
    pub flag_verbose: bool,
    pub flag_host: String,
    pub flag_port: String,
    pub flag_base_path: String,
    pub flag_security_level: String,
    pub flag_chain: String,
    pub cmd_server: bool,
    pub cmd_list: bool,
    pub cmd_new: bool,
    pub cmd_hide: bool,
    pub cmd_unhide: bool,
    pub cmd_update: bool,
    pub cmd_strip: bool,
    pub cmd_import: bool,
    pub cmd_export: bool,
    pub cmd_transaction: bool,
}

type ExecResult<Error> = Result<(), Error>;

pub struct CmdExecutor;

impl CmdExecutor {
    ///
    pub fn run(args: &Args) -> ExecResult<Error> {
        if args.cmd_server {
            CmdExecutor::server(&args)
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
        } else {
            Err(Error::ExecError(
                "No command selected. Use `-h` to see help menu".to_string(),
            ))
        }

    }

    ///
    fn server(args: &Args) -> ExecResult<Error> {
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

        Ok(())
    }

    ///
    fn list() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn new_account() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn hide() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn unhide() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn strip() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn export() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn import() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn update() -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn sign_transaction() -> ExecResult<Error> {
        Ok(())
    }
}
