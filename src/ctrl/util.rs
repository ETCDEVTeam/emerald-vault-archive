//! # Helpers for command execution


use super::Error;
use super::{CmdExecutor, Address, PrivateKey, KeyFile};
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use std::fs::File;
use std::str::FromStr;
use std::env;
use rpc::{ClientMethod, MethodParams, ClientMethod};
use jsonrpc_core::Params;


#[macro_export]
macro_rules! arg_to_opt {
    ( $arg:expr ) => {{
        let str = $arg.parse::<String>()?;
        if str.is_empty() {
            None
        } else {
            Some(str)
        }
    }};
}


macro_rules! arg_to_address {
    ( $arg:expr ) => {{
        let str = $arg.parse::<String>()?;
        Address::from_str(&str)?
    }};
}

/// Environment variables used to change default variables
#[derive(Default, Debug)]
struct EnvVars {
    emerald_base_path: Option<String>,
    emerald_host: Option<String>,
    emerald_port: Option<String>,
    emerald_chain: Option<String>,
    emerald_chain_id: Option<String>,
    emerald_gas: Option<String>,
    emerald_gas_cost: Option<String>,
    emerald_security_level: Option<String>,
    emerald_node: Option<String>,
}

impl EnvVars {
    /// Collect environment variables to overwrite default values
    pub fn new() -> EnvVars {
        let mut vars = EnvVars::default();
        for (key, value) in env::vars() {
            match key.as_ref() {
                "EMERALD_BASE_PATH" => vars.emerald_base_path = Some(value),
                "EMERALD_HOST" => vars.emerald_host = Some(value),
                "EMERALD_PORT" => vars.emerald_port = Some(value),
                "EMERALD_CHAIN" => vars.emerald_chain = Some(value),
                "EMERALD_CHAIN_ID" => vars.emerald_chain_id = Some(value),
                "EMERALD_GAS" => vars.emerald_gas = Some(value),
                "EMERALD_GAS_COST" => vars.emerald_gas_cost = Some(value),
                "EMERALD_SECURITY_LEVEL" => vars.emerald_security_level = Some(value),
                "EMERALD_NODE" => vars.emerald_node = Some(value),
                _ => (),
            }
        }
        vars
    }
}

impl CmdExecutor {
    /// Import Keyfile into storage
    pub fn import_keyfile<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut json = String::new();
        File::open(path).and_then(
            |mut f| f.read_to_string(&mut json),
        )?;

        let kf = KeyFile::decode(json)?;
        self.storage.put(&kf)?;
        //        io::stdout().write_all(
        //            &format!("kf: {:?}\n", kf).into_bytes(),
        //        )?;

        Ok(())
    }

    /// Parse `address` from input arguments
    pub fn parse_address(&self) -> Result<Address, Error> {
        Ok(arg_to_address!(self.args.arg_address))
    }

    /// Parse `from` address for transaction signing
    pub fn parse_from(&self) -> Result<Address, Error> {
        Ok(arg_to_address!(self.args.arg_from))
    }

    /// Parse `to` address for transaction signing
    pub fn parse_to(&self) -> Result<Option<Address>, Error> {
        let str = self.args.arg_to.parse::<String>()?;
        let val = if str.is_empty() {
            None
        } else {
            Some(Address::from_str(&str)?)
        };

        Ok(val)
    }

    /// Parse private key for account creation
    pub fn parse_pk(&self) -> Result<PrivateKey, Error> {
        let pk_str = self.args.arg_path.parse::<String>()?;
        let pk = PrivateKey::from_str(&pk_str)?;

        Ok(pk)
    }

    /// Parse path for accounts import/export
    pub fn parse_path(&self) -> Result<PathBuf, Error> {
        let pk_str = self.args.arg_path.parse::<String>()?;
        let pk = PathBuf::from(&pk_str);

        Ok(pk)
    }

    /// Request passphrase
    pub fn request_passphrase() -> Result<String, Error> {
        let mut out = io::stdout();
        out.write_all(b"Enter passphrase: \n")?;
        out.flush()?;

        let mut passphrase = String::new();
        io::stdin().read_line(&mut passphrase)?;

        Ok(passphrase)
    }

    /// Get nonce for address from remote node
    ///
    /// # Arguments:
    ///
    /// * addr - target address
    ///
    pub fn get_nonce(&self, client_addr: &SocketAddr) -> Result<u64, Error> {
        let url = Connector::new(&format!("http://{}", client_addr));

        url.send_post(&MethodParams(ClientMethod::EthGetTxCount, &p));
        Ok(0u64)
    }


    ///
    pub fn send_transaction() -> Result<(), Error> {

    }
}
