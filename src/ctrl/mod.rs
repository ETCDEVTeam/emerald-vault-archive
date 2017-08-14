//! # Command executor

mod error;

use super::emerald::keystore::{KeyFile, KdfDepthLevel};
use super::emerald::{self, Address};
use super::emerald::storage::{KeyfileStorage, build_storage, default_keystore_path};
use super::log::LogLevel;
use self::error::Error;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Write};


#[derive(Debug, Deserialize, Clone)]
pub struct Args {
    pub arg_address: String,
    pub arg_name: String,
    pub arg_description: String,
    pub flag_version: bool,
    pub flag_quiet: bool,
    pub flag_verbose: bool,
    pub flag_host: String,
    pub flag_port: String,
    pub flag_base_path: String,
    pub flag_security_level: String,
    pub flag_chain: String,
    pub flag_name: String,
    pub flag_description: String,
    pub flag_show_hidden: bool,
    pub flag_all: bool,
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

        let sec_level_str: &str = &args.flag_security_level.parse::<String>()?;
        let sec_level = match KdfDepthLevel::from_str(sec_level_str) {
            Ok(sec) => sec,
            Err(e) => {
                error!("{}", e.to_string());
                KdfDepthLevel::default()
            }
        };

        let base_path_str = args.flag_base_path.parse::<String>()?;
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
            self.list()
        } else if self.args.cmd_new {
            self.new_account()
        } else if self.args.cmd_hide {
            self.hide()
        } else if self.args.cmd_unhide {
            self.unhide()
        } else if self.args.cmd_update {
            self.update()
        } else if self.args.cmd_strip {
            self.strip()
        } else if self.args.cmd_import {
            self.import()
        } else if self.args.cmd_export {
            self.export()
        } else if self.args.cmd_transaction {
            self.sign_transaction()
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
            .parse::<SocketAddr>()?;

        emerald::rpc::start(
            &addr,
            &self.chain,
            self.base_path.clone(),
            Some(self.sec_level),
        );

        Ok(())
    }

    ///
    fn list(&self) -> ExecResult<Error> {
        let accounts = self.storage.list_accounts(self.args.flag_show_hidden)?;

        io::stdout().write(&format!("Total: {}\n", accounts.len())
            .into_bytes())?;
        for v in accounts.into_iter() {
            io::stdout().write(&format!(
                "Account: {}, name: {}, description: {}\n",
                &v.1,
                &v.0,
                &v.2
            ).into_bytes())?;
        }
        io::stdout().flush()?;

        Ok(())
    }

    ///
    fn new_account(&self) -> ExecResult<Error> {
        let mut out = io::stdout();
        out.write(
            b"! Warning: passphrase can't be restored. Don't forget it !\n",
        )?;
        out.write(b"Enter strong password: \n")?;
        out.flush()?;

        let mut passphrase = String::new();
        io::stdin().read_line(&mut passphrase)?;

        let name_str = self.args.arg_name.parse::<String>()?;
        let name = match name_str.is_empty() {
            true => None,
            _ => Some(name_str),
        };

        let desc_str = self.args.arg_description.parse::<String>()?;
        let desc = match desc_str.is_empty() {
            true => None,
            _ => Some(desc_str),
        };

        let kf = KeyFile::new(&passphrase, &self.sec_level, name, desc)?;
        self.storage.put(&kf)?;
        io::stdout().write(&format!(
            "Created new account: {}",
            &kf.address.to_string()
        ).into_bytes())?;
        out.flush()?;

        Ok(())
    }

    ///
    fn hide(&self) -> ExecResult<Error> {
        let address = self.parse_address()?;
        self.storage.hide(&address)?;

        Ok(())
    }

    ///
    fn unhide(&self) -> ExecResult<Error> {
        let address = self.parse_address()?;
        self.storage.unhide(&address)?;

        Ok(())
    }

    ///
    fn strip(&self) -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn export(&self) -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn import(&self) -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn update(&self) -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn sign_transaction(&self) -> ExecResult<Error> {
        Ok(())
    }

    ///
    fn parse_address(&self) -> Result<Address, Error> {
        let addr_str = self.args.arg_address.parse::<String>()?;
        let add = Address::from_str(&addr_str)?;

        Ok(add)
    }
}
