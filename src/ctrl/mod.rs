//! # Command executor

mod error;
#[macro_use]
mod util;

pub use self::error::Error;
use super::emerald::keystore::{KeyFile, KdfDepthLevel};
use super::emerald::{self, Address, Transaction, to_arr, to_chain_id, trim_hex, align_bytes,
                     to_u64, to_even_str};
use super::emerald::PrivateKey;
use super::emerald::storage::{KeyfileStorage, build_storage, default_keystore_path};
use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Write};
use std::fs;
use std::path::PathBuf;
use rustc_serialize::json;
use std::sync::Arc;
use self::util::*;
use rpc::Connector;
use hex::ToHex;


#[derive(Debug, Deserialize, Clone)]
pub struct Args {
    pub arg_address: String,
    pub arg_path: String,
    pub arg_from: String,
    pub arg_to: String,
    pub arg_value: String,
    pub arg_key: String,
    pub flag_raw: bool,
    pub flag_version: bool,
    pub flag_quiet: bool,
    pub flag_verbose: bool,
    pub flag_host: String,
    pub flag_port: String,
    pub flag_nonce: String,
    pub flag_gas: String,
    pub flag_gas_price: String,
    pub flag_data: String,
    pub flag_base_path: String,
    pub flag_security_level: String,
    pub flag_chain: String,
    pub flag_name: String,
    pub flag_description: String,
    pub flag_upstream: String,
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
    storage: Arc<Box<KeyfileStorage>>,
    args: Args,
    vars: EnvVars,
    connector: Option<Connector>,
}

impl CmdExecutor {
    /// Create new command executor
    pub fn new(args: &Args) -> Result<Self, Error> {
        let env = EnvVars::parse();

        let chain = match arg_or_default(&args.flag_chain, &env.emerald_chain) {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e.to_string());
                "mainnet".to_string()
            }
        };

        let sec_level_str = arg_or_default(&args.flag_security_level, &env.emerald_security_level)?;
        let sec_level = match KdfDepthLevel::from_str(&sec_level_str) {
            Ok(sec) => sec,
            Err(e) => {
                error!("{}", e.to_string());
                KdfDepthLevel::default()
            }
        };

        let keystore_path = match arg_or_default(&args.flag_base_path, &env.emerald_base_path) {
            Ok(ref path) => PathBuf::from(path),
            Err(_) => default_keystore_path(&chain),
        };
        let storage = build_storage(keystore_path)?;

        let connector = match args.flag_upstream.parse::<SocketAddr>() {
            Ok(addr) => Some(Connector::new(&format!("http://{}", addr))),
            Err(_) => None,
        };


        Ok(CmdExecutor {
            args: args.clone(),
            chain: chain,
            sec_level: sec_level,
            storage: Arc::new(storage),
            vars: env,
            connector: connector,
        })
    }

    /// Dispatch command to proper handler
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
                "No command selected. Use `-h` for help".to_string(),
            ))
        }

    }

    /// Launch connector in a `server` mode
    fn server(&self) -> ExecResult<Error> {
        info!("Starting Emerald Connector - v{}", emerald::version());
        info!("Chain set to '{}'", self.chain);
        info!("Security level set to '{}'", self.sec_level);

        let addr = format!("{}:{}", self.args.flag_host, self.args.flag_port)
            .parse::<SocketAddr>()?;

        emerald::rpc::start(
            &addr,
            &self.chain,
            self.storage.clone(),
            Some(self.sec_level),
        );

        Ok(())
    }

    /// List all accounts
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

    /// Creates new account
    fn new_account(&self) -> ExecResult<Error> {
        let mut out = io::stdout();
        out.write_all(
            b"! Warning: passphrase can't be restored. Don't forget it !\n",
        )?;
        let passphrase = request_passphrase()?;
        let name = arg_to_opt!(self.args.flag_name);
        let desc = arg_to_opt!(self.args.flag_description);

        let kf = if self.args.flag_raw {
            let pk = parse_pk(&self.args.arg_key)?;
            let mut kf = KeyFile::new(&passphrase, &self.sec_level, name, desc)?;
            kf.encrypt_key(pk, &passphrase);
            kf
        } else {
            KeyFile::new(&passphrase, &self.sec_level, name, desc)?
        };

        self.storage.put(&kf)?;
        out.write_all(&format!(
            "Created new account: {}",
            &kf.address.to_string()
        ).into_bytes())?;
        out.flush()?;

        Ok(())
    }

    /// Hide account from being listed
    fn hide(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        self.storage.hide(&address)?;

        Ok(())
    }

    /// Unhide account from being listed
    fn unhide(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        self.storage.unhide(&address)?;

        Ok(())
    }

    /// Extract private key from a keyfile
    fn strip(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        let kf = self.storage.search_by_address(&address)?;
        let passphrase = request_passphrase()?;
        let pk = kf.decrypt_key(&passphrase)?;

        io::stdout().write_all(
            &format!("Private key: {}", &pk.to_string())
                .into_bytes(),
        )?;
        io::stdout().flush()?;

        Ok(())
    }

    /// Export accounts
    fn export(&self) -> ExecResult<Error> {
        let path = parse_path_or_default(&self.args.flag_base_path, &self.vars.emerald_base_path)?;

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
                p.push(&info.filename);

                let json = json::encode(&kf).and_then(|s| Ok(s.into_bytes()))?;
                let mut f = fs::File::create(p)?;
                f.write_all(&json)?;
            }
        }

        if path.is_file() {};

        Ok(())
    }

    /// Import accounts
    fn import(&self) -> ExecResult<Error> {
        let path = parse_path_or_default(&self.args.flag_base_path, &self.vars.emerald_base_path)?;

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

    /// Update `name` and `description` for existing account
    fn update(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        let name = arg_to_opt!(self.args.flag_name);
        let desc = arg_to_opt!(self.args.flag_description);

        self.storage.update(&address, name, desc)?;

        Ok(())
    }

    /// Sign transaction
    fn sign_transaction(&self) -> ExecResult<Error> {
        let from = parse_address(&self.args.arg_from)?;
        let kf = self.storage.search_by_address(&from)?;

        let tr = Transaction {
            nonce: self.get_nonce(&from)?,
            gas_price: parse_gas_price_or_default(
                &self.args.flag_gas_price,
                &self.vars.emerald_gas_price,
            )?,
            gas_limit: parse_gas_or_default(&self.args.flag_gas, &self.vars.emerald_gas)?,
            to: match parse_address(&self.args.arg_to) {
                Ok(a) => Some(a),
                Err(_) => None,
            },
            value: parse_value(&self.args.arg_value)?,
            data: parse_data(&self.args.flag_data)?,
        };
        let pass = request_passphrase()?;
        let pk = kf.decrypt_key(&pass)?;

        if let Some(chain_id) = to_chain_id(&self.chain) {
            let raw = tr.to_signed_raw(pk, chain_id)?;

            println!("Signed transaction: ");
            println!("{}", raw.to_hex());

            if self.connector.is_some() {
                let tx_hash = self.send_transaction(raw)?;
                println!("Tx hash: ");
                println!("{}", tx_hash);
            }

            Ok(())
        } else {
            Err(Error::ExecError("Invalid chain name".to_string()))
        }
    }
}
