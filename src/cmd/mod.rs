//! # Command executor

mod error;
#[macro_use]
mod arg_handlers;

pub use self::error::Error;
use super::emerald::keystore::{KeyFile, KdfDepthLevel};
use super::emerald::{self, Address, Transaction, to_arr, to_chain_id, trim_hex, align_bytes,
                     to_even_str};
use super::emerald::PrivateKey;
use super::emerald::mnemonic::{Mnemonic, Language, ENTROPY_BYTE_LENGTH, gen_entropy};
use super::emerald::storage::{default_path, StorageController};
use std::net::SocketAddr;
use std::str::FromStr;
use std::fs;
use self::arg_handlers::*;
use rpc::{self, RpcConnector};
use hex::ToHex;
use std::path::PathBuf;
use std::sync::Arc;


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
    pub flag_force: bool,
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
    pub cmd_mnemonic: bool,
    pub cmd_balance: bool,
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
    storage_ctrl: Arc<Box<StorageController>>,
    args: Args,
    vars: EnvVars,
    connector: Option<RpcConnector>,
}

impl CmdExecutor {
    /// Create new command executor
    pub fn new(args: &Args) -> Result<Self, Error> {
        let env = EnvVars::parse();

        let chain = match arg_or_default(&args.flag_chain, &env.emerald_chain) {
            Ok(c) => c,
            Err(e) => {
                info!("Missed `--chain` argument. Use default: `mainnet`");
                "mainnet".to_string()
            }
        };

        let sec_level_str = arg_or_default(&args.flag_security_level, &env.emerald_security_level)?;
        let sec_level = match KdfDepthLevel::from_str(&sec_level_str) {
            Ok(sec) => sec,
            Err(e) => {
                info!("Missed `--security-level` argument. Use default: `ultra`");
                KdfDepthLevel::default()
            }
        };

        let mut p = PathBuf::new();
        let base_path = match arg_or_default(&args.flag_base_path, &env.emerald_base_path) {
            Ok(path) => {
                p.push(&path);
                p
            }
            Err(_) => default_path(),
        };
        let storage_ctrl = Arc::new(Box::new(StorageController::new(base_path)?));

        let connector = match args.flag_upstream.parse::<SocketAddr>() {
            Ok(addr) => Some(RpcConnector::new(&format!("http://{}", addr))),
            Err(_) => None,
        };


        Ok(CmdExecutor {
            args: args.clone(),
            chain: chain,
            sec_level: sec_level,
            storage_ctrl: storage_ctrl,
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
        } else if self.args.cmd_mnemonic {
            self.new_mnemonic()
        } else if self.args.cmd_balance {
            self.balance()
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
            let (kf, mut tr) = self.build_transaction()?;
            let pass = request_passphrase()?;
            let pk = kf.decrypt_key(&pass)?;

            let raw = self.sign_transaction(&tr, pk)?;
            match self.connector {
                Some(ref conn) => {
                    tr.nonce = rpc::get_nonce(conn, &kf.address)?;
                    self.send_transaction(&raw)
                }
                None => {
                    println!("Signed transaction: ");
                    println!("{}", raw.to_hex());
                    Ok(())
                }
            }
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

        let storage_ctrl = Arc::clone(&self.storage_ctrl);
        emerald::rpc::start(&addr, &self.chain, storage_ctrl, Some(self.sec_level));

        Ok(())
    }

    /// List all accounts
    fn list(&self) -> ExecResult<Error> {
        let st = self.storage_ctrl.get_keystore(&self.chain)?;
        let accounts_info = st.list_accounts(self.args.flag_show_hidden)?;

        println!("{0: <45} {1: <45} ", "ADDRESS", "NAME");
        for info in accounts_info {
            println!("{0: <45} {1: <45} ", &info.address, &info.name);
        }

        Ok(())
    }

    /// Creates new account
    fn new_account(&self) -> ExecResult<Error> {
        println!("! Warning: passphrase can't be restored. Don't forget it !");
        let passphrase = request_passphrase()?;
        let name = arg_to_opt(&self.args.flag_name)?;
        let desc = arg_to_opt(&self.args.flag_description)?;

        let kf = if self.args.flag_raw {
            let pk = parse_pk(&self.args.arg_key)?;
            let mut kf = KeyFile::new(&passphrase, &self.sec_level, name, desc)?;
            kf.encrypt_key(pk, &passphrase);
            kf
        } else {
            KeyFile::new(&passphrase, &self.sec_level, name, desc)?
        };

        let st = self.storage_ctrl.get_keystore(&self.chain)?;
        st.put(&kf)?;
        println!("Created new account: {}", &kf.address.to_string());

        Ok(())
    }

    /// Creates new BIP32 mnemonic phrase
    fn new_mnemonic(&self) -> ExecResult<Error> {
        let entropy = gen_entropy(ENTROPY_BYTE_LENGTH)?;
        let mn = Mnemonic::new(Language::English, &entropy)?;
        println!("{}", mn.sentence());
    }

    /// Show user balance
    fn balance(&self) -> ExecResult<Error> {
        match self.connector {
            Some(ref conn) => {
                let address = parse_address(&self.args.arg_address)?;
                let balance = rpc::get_balance(conn, &address)?;
                println!("Address: {}, balance: {}", &address, &balance);

                Ok(())
            }
            None => Err(Error::ExecError("no connection to client".to_string())),
        }
    }

    /// Hide account from being listed
    fn hide(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        let st = self.storage_ctrl.get_keystore(&self.chain)?;
        st.hide(&address)?;

        Ok(())
    }

    /// Unhide account from being listed
    fn unhide(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        let st = self.storage_ctrl.get_keystore(&self.chain)?;
        st.unhide(&address)?;

        Ok(())
    }

    /// Extract private key from a keyfile
    fn strip(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        let st = self.storage_ctrl.get_keystore(&self.chain)?;

        let (_, kf) = st.search_by_address(&address)?;
        let passphrase = request_passphrase()?;
        let pk = kf.decrypt_key(&passphrase)?;

        println!("Private key: {}", &pk.to_string());

        Ok(())
    }

    /// Export accounts
    fn export(&self) -> ExecResult<Error> {
        let path = parse_path_or_default(&self.args.arg_path, &self.vars.emerald_base_path)?;

        if self.args.flag_all {
            if !path.is_dir() {
                return Err(Error::ExecError(
                    "`export`: invalid args. Use `-h` for help.".to_string(),
                ));
            }

            let st = self.storage_ctrl.get_keystore(&self.chain)?;
            let accounts_info = st.list_accounts(true)?;
            for info in accounts_info {
                let addr = Address::from_str(&info.address)?;
                self.export_keyfile(&addr, &path)?
            }
        } else {
            let addr = parse_address(&self.args.arg_address)?;
            self.export_keyfile(&addr, &path)?
        }

        Ok(())
    }

    /// Import accounts
    fn import(&self) -> ExecResult<Error> {
        let path = parse_path_or_default(&self.args.arg_path, &self.vars.emerald_base_path)?;
        let mut counter = 0;

        if path.is_file() {
            self.import_keyfile(path, self.args.flag_force)?;
            counter += 1;
        } else {
            let entries = fs::read_dir(&path)?;
            for entry in entries {
                let path = entry?.path();
                if path.is_dir() {
                    continue;
                }
                self.import_keyfile(path, self.args.flag_force)?;
                counter += 1;
            }
        }

        println!("Imported accounts: {}", counter);

        Ok(())
    }

    /// Update `name` and `description` for existing account
    fn update(&self) -> ExecResult<Error> {
        let address = parse_address(&self.args.arg_address)?;
        let name = arg_to_opt(&self.args.flag_name)?;
        let desc = arg_to_opt(&self.args.flag_description)?;

        let st = self.storage_ctrl.get_keystore(&self.chain)?;
        st.update(&address, name, desc)?;

        Ok(())
    }

    /// Sign transaction with
    fn sign_transaction(&self, tr: &Transaction, pk: PrivateKey) -> Result<Vec<u8>, Error> {
        if let Some(chain_id) = to_chain_id(&self.chain) {
            let raw = tr.to_signed_raw(pk, chain_id)?;
            Ok(raw)
        } else {
            Err(Error::ExecError("Invalid chain name".to_string()))
        }
    }

    /// Send transaction into network through provided node
    fn send_transaction(&self, raw: &[u8]) -> ExecResult<Error> {
        match self.connector {
            Some(ref conn) => {
                let tx_hash = rpc::send_transaction(conn, raw)?;
                println!("Tx hash: ");
                println!("{}", tx_hash);
                Ok(())
            }

            None => Err(Error::ExecError("Can't connect to node".to_string())),
        }
    }
}
