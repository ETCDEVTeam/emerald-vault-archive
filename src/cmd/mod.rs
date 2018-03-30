//! # Command executor

mod account;
mod transaction;
mod error;
#[macro_use]
mod arg_handlers;

pub use self::error::Error;
pub use self::arg_handlers::*;
use self::account::account_cmd;
use super::emerald::keystore::{KdfDepthLevel, KeyFile};
use super::emerald::{self, align_bytes, to_arr, to_chain_id, to_even_str, trim_hex, Address,
                     Transaction};
use super::emerald::PrivateKey;
use super::emerald::mnemonic::{gen_entropy, Language, Mnemonic, ENTROPY_BYTE_LENGTH};
use super::emerald::storage::{default_path, KeyfileStorage, StorageController};
use std::net::SocketAddr;
use std::str::FromStr;
use std::fs;
use rpc::{self, RpcConnector};
use hex::ToHex;
use std::path::PathBuf;
use std::sync::Arc;
use std::io::{self, Read};
use clap::ArgMatches;

type ExecResult = Result<(), Error>;

const DEFAULT_CHAIN_NAME: &str = "mainnet";
const DEFAULT_UPSTREAM: &str = "localhost:8545";

pub struct CmdExecutor<'a> {
    chain: &'a str,
    storage_ctrl: Arc<Box<StorageController>>,
    matches: &'a ArgMatches<'a>,
    env: EnvVars,
    connector: Option<RpcConnector>,
}

impl<'a> CmdExecutor<'a> {
    /// Create new command executor
    pub fn new(matches: &'a ArgMatches<'a>) -> Result<Self, Error> {
        let env = EnvVars::parse();

        let chain = matches.value_of("chain").unwrap_or(DEFAULT_CHAIN_NAME);
        info!("Chain name: {}", DEFAULT_CHAIN_NAME);

        let mut base_path = PathBuf::new();
        if let Some(p) = matches
            .value_of("base-path")
            .or(env.emerald_base_path.as_ref().map(String::as_str))
        {
            base_path.push(&p)
        } else {
            base_path = default_path();
        }
        let storage_ctrl = Arc::new(Box::new(StorageController::new(base_path)?));

        let ups = matches.value_of("upstream").unwrap_or(DEFAULT_UPSTREAM);
        let connector = parse_socket(&ups)
            .or_else(|_| parse_url(&ups))
            .and_then(RpcConnector::new)
            .ok();

        Ok(CmdExecutor {
            matches,
            chain,
            storage_ctrl,
            env,
            connector,
        })
    }

    /// Dispatch command to proper handler
    pub fn run(&self) -> ExecResult {
        match self.matches.subcommand() {
            ("server", Some(sub_m)) => Ok(()),
            ("account", Some(sub_m)) => {
                let keystore = self.storage_ctrl.get_keystore(self.chain)?;
                account_cmd(sub_m, keystore, &self.env)
            }
            ("balance", Some(sub_m)) => Ok(()),
            ("mnemonic", Some(sub_m)) => Ok(()),
            ("transaction", Some(sub_m)) => Ok(()),
            _ => Err(Error::ExecError(
                "No command selected. Use `-h` for help".to_string(),
            )),
        }
    }

    //    /// Launch connector in a `server` mode
    //    fn server(&self) -> ExecResult {
    //        info!("Starting Emerald Connector - v{}", emerald::version());
    //        info!("Chain set to '{}'", self.chain);
    //        info!("Security level set to '{}'", self.sec_level);
    //
    //        let addr =
    //            format!("{}:{}", self.args.flag_host, self.args.flag_port).parse::<SocketAddr>()?;
    //
    //        let storage_ctrl = Arc::clone(&self.storage_ctrl);
    //        emerald::rpc::start(&addr, &self.chain, storage_ctrl, Some(self.sec_level));
    //
    //        Ok(())
    //    }
    //
    //
    //    /// Show user balance
    //    fn balance(&self) -> ExecResult {
    //        match self.connector {
    //            Some(ref conn) => {
    //                let address = parse_address(&self.args.arg_address)?;
    //                let balance = rpc::get_balance(conn, &address)?;
    //                println!("Address: {}, balance: {}", &address, &balance);
    //
    //                Ok(())
    //            }
    //            None => Err(Error::ExecError("no connection to client".to_string())),
    //        }
    //    }
    //
    //    /// Creates new BIP32 mnemonic phrase
    //    fn new_mnemonic(&self) -> ExecResult {
    //        let entropy = gen_entropy(ENTROPY_BYTE_LENGTH)?;
    //        let mn = Mnemonic::new(Language::English, &entropy)?;
    //        println!("{}", mn.sentence());
    //        Ok(())
    //    }
}
