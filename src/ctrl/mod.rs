//! # Command executor

mod error;

use super::emerald::keystore::KdfDepthLevel;
use super::emerald;
use super::emerald::storage::{KeyfileStorage, build_storage, default_keystore_path};
use super::log::LogLevel;
use self::error::Error;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::str::FromStr;


#[derive(Debug, Deserialize, Clone)]
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

pub struct CmdExecutor {
    chain: String,
    sec_level: KdfDepthLevel,
    base_path: Option<PathBuf>,
    storage: Box<KeyfileStorage>,
    args: Args,
}

impl CmdExecutor {
    ///
    pub fn new(args: &Args) -> Result<Self, Error> {
        let chain = match args.flag_chain.parse::<String>() {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e.to_string());
                "mainnet".to_string()
            }
        };

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

        let base_path_str = args.flag_base_path.parse::<String>().expect(
            "Expect to parse base \
             path",
        );


        let base_path = if !base_path_str.is_empty() {
            Some(PathBuf::from(&base_path_str))
        } else {
            None
        };

        let keystore_path = default_keystore_path(&chain);
        let storage = build_storage(keystore_path)?;

        Ok(CmdExecutor {
            args: args.clone(),
            chain: chain,
            base_path: base_path,
            sec_level: sec_level,
            storage: storage,
        })
    }

    ///
    pub fn run(&self) -> ExecResult<Error> {
        if self.args.cmd_server {
            self.server()
        } else if self.args.cmd_list {
            CmdExecutor::list()
        } else if self.args.cmd_new {
            CmdExecutor::new_account()
        } else if self.args.cmd_hide {
            CmdExecutor::hide()
        } else if self.args.cmd_unhide {
            CmdExecutor::unhide()
        } else if self.args.cmd_update {
            CmdExecutor::update()
        } else if self.args.cmd_strip {
            CmdExecutor::strip()
        } else if self.args.cmd_import {
            CmdExecutor::import()
        } else if self.args.cmd_export {
            CmdExecutor::export()
        } else if self.args.cmd_transaction {
            CmdExecutor::sign_transaction()
        } else {
            Err(Error::ExecError(
                "No command selected. Use `-h` to see help menu".to_string(),
            ))
        }

    }

    ///
    fn server(&self) -> ExecResult<Error> {
        if log_enabled!(LogLevel::Info) {
            info!("Starting Emerald Connector - v{}", emerald::version());
        }

        info!("Chain set to '{}'", self.chain);
        info!("Security level set to '{}'", self.sec_level);

        let addr = format!("{}:{}", self.args.flag_host, self.args.flag_port)
            .parse::<SocketAddr>()
            .expect("Expect to parse address");

        emerald::rpc::start(
            &addr,
            &self.chain,
            self.base_path.clone(),
            Some(self.sec_level),
        );

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
