//! # Command executor

mod error;
mod util;

use super::emerald::keystore::{KeyFile, KdfDepthLevel};
use super::emerald::{self, Address};
use super::emerald::PrivateKey;
use super::emerald::storage::{KeyfileStorage, build_storage, default_keystore_path,
                              generate_filename};
use super::log::LogLevel;
use self::error::Error;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Write};
use std::fs;
use rustc_serialize::json;


#[derive(Debug, Deserialize, Clone)]
pub struct Args {
    pub arg_address: String,
    pub arg_path: String,
    pub arg_name: String,
    pub arg_description: String,
    pub arg_key: String,
    pub flag_raw: bool,
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
        let accounts_info = self.storage.list_accounts(self.args.flag_show_hidden)?;

        io::stdout().write_all(
            &format!("Total: {}\n", accounts_info.len())
                .into_bytes(),
        )?;

        for info in accounts_info {
            io::stdout().write_all(&format!(
                "Account: {}, name: {}, description: {}\n",
                &info.address,
                &info.name,
                &info.description
            ).into_bytes())?;
        }
        io::stdout().flush()?;

        Ok(())
    }

    ///
    fn new_account(&self) -> ExecResult<Error> {
        let mut out = io::stdout();
        out.write_all(
            b"! Warning: passphrase can't be restored. Don't forget it !\n",
        )?;
        out.write_all(b"Enter passphrase: \n")?;
        out.flush()?;

        let mut passphrase = String::new();
        io::stdin().read_line(&mut passphrase)?;

        let name_str = self.args.arg_name.parse::<String>()?;
        let name = if name_str.is_empty() {
            None
        } else {
            Some(name_str)
        };

        let desc_str = self.args.arg_description.parse::<String>()?;
        let desc = if desc_str.is_empty() {
            None
        } else {
            Some(desc_str)
        };

        let kf = if self.args.flag_raw {
            let pk = self.parse_pk()?;
            let mut kf = KeyFile::new(&passphrase, &self.sec_level, name, desc)?;
            kf.encrypt_key(pk, &passphrase);
            kf
        } else {
            KeyFile::new(&passphrase, &self.sec_level, name, desc)?
        };

        self.storage.put(&kf)?;
        io::stdout().write_all(&format!(
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
        let address = self.parse_address()?;
        let kf = self.storage.search_by_address(&address)?;
        let passphrase = CmdExecutor::request_passphrase()?;
        let pk = kf.decrypt_key(&passphrase)?;

        io::stdout().write_all(
            &format!("Private key: {}", &pk.to_string())
                .into_bytes(),
        )?;
        io::stdout().flush()?;

        Ok(())
    }

    ///
    fn export(&self) -> ExecResult<Error> {
        let path = self.parse_path()?;

        if self.args.flag_all {
            if !path.is_dir() {
                return Err(Error::ExecError(
                    "`export`: invalid args. Use `-h` for help.".to_string(),
                ));
            }

            let accounts_info = self.storage.list_accounts(true)?;
            for info in accounts_info {
                let address = Address::from_str(&info.address)?;
                let kf = self.storage.search_by_address(&address)?;

                let mut p = path.clone();
                p.push(&generate_filename(&kf.uuid.to_string()));

                let json = json::encode(&kf).and_then(|s| Ok(s.into_bytes()))?;
                let mut f = fs::File::create(p)?;
                f.write_all(&json)?;
            }
        }

        if path.is_file() {};

        Ok(())
    }

    ///
    fn import(&self) -> ExecResult<Error> {
        let path = self.parse_path()?;

        if path.is_file() {
            self.import_keyfile(path)?;
        } else {
            let entries = fs::read_dir(&path)?;
            for entry in entries {
                let path = entry?.path();
                if path.is_dir() {
                    continue;
                }
                self.import_keyfile(path)?;
            }
        }

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
}
