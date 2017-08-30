//! # Helpers for command execution


use super::Error;
use super::{CmdExecutor, Address, PrivateKey, KeyFile, trim_hex, to_arr, align_bytes, to_u64,
            to_even_str};
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::env;
use rpc::{ClientMethod, MethodParams};
use jsonrpc_core::Params;
use serde_json::Value;
use rustc_serialize::hex::ToHex;
use hex::FromHex;

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

/// Environment variables used to change default variables
#[derive(Default, Debug)]
pub struct EnvVars {
    pub emerald_base_path: Option<String>,
    pub emerald_host: Option<String>,
    pub emerald_port: Option<String>,
    pub emerald_chain: Option<String>,
    pub emerald_chain_id: Option<String>,
    pub emerald_gas: Option<String>,
    pub emerald_gas_price: Option<String>,
    pub emerald_security_level: Option<String>,
    pub emerald_node: Option<String>,
}

impl EnvVars {
    /// Collect environment variables to overwrite default values
    pub fn parse() -> EnvVars {
        let mut vars = EnvVars::default();
        for (key, value) in env::vars() {
            match key.as_ref() {
                "EMERALD_BASE_PATH" => vars.emerald_base_path = Some(value),
                "EMERALD_HOST" => vars.emerald_host = Some(value),
                "EMERALD_PORT" => vars.emerald_port = Some(value),
                "EMERALD_CHAIN" => vars.emerald_chain = Some(value),
                "EMERALD_CHAIN_ID" => vars.emerald_chain_id = Some(value),
                "EMERALD_GAS" => vars.emerald_gas = Some(value),
                "EMERALD_GAS_PRICE" => vars.emerald_gas_price = Some(value),
                "EMERALD_SECURITY_LEVEL" => vars.emerald_security_level = Some(value),
                "EMERALD_NODE" => vars.emerald_node = Some(value),
                _ => (),
            }
        }
        vars
    }
}

/// Try to parse argument
/// If no `arg` was supplied, try to get environment variable
///
/// # Arguments:
///
/// * arg - provided argument
/// * env - optional environment variable
///
pub fn arg_or_default(arg: &str, env: &Option<String>) -> Result<String, Error> {
    let val = arg.parse::<String>()?;

    if val.is_empty() {
        env.clone().ok_or_else(|| {
            Error::ExecError("Missed arguments".to_string())
        })
    } else {
        Ok(val)
    }
}

/// Converts hex string to 32 bytes array
/// Aligns original `hex` to fit 32 bytes
fn hex_to_32bytes(hex: &str) -> Result<[u8; 32], Error> {
    let bytes = Vec::from_hex(to_even_str(trim_hex(hex)))?;
    Ok(to_arr(&align_bytes(&bytes, 32)))
}

/// Parse `address` from input arguments
pub fn parse_address(s: &str) -> Result<Address, Error> {
    let str = s.parse::<String>()?;
    Ok(Address::from_str(&str)?)
}

/// Parse private key for account creation
pub fn parse_pk(s: &str) -> Result<PrivateKey, Error> {
    let pk_str = s.parse::<String>()?;
    let pk = PrivateKey::from_str(&pk_str)?;
    Ok(pk)
}


/// Parse transaction value
pub fn parse_value(s: &str) -> Result<[u8; 32], Error> {
    let value_str = s.parse::<String>()?;
    hex_to_32bytes(&value_str)
}

/// Parse transaction data
pub fn parse_data(s: &str) -> Result<Vec<u8>, Error> {
    let str = s.parse::<String>()?;
    let data = Vec::from_hex(to_even_str(trim_hex(&str)))?;
    Ok(data)
}

/// Parse transaction data
pub fn parse_nonce(s: &str) -> Result<u64, Error> {
    let nonce_str = s.parse::<String>()?;
    Ok(u64::from_str_radix(&to_even_str(trim_hex(&nonce_str)), 16)?)
}

/// Parse path for accounts import/export
pub fn parse_path_or_default(s: &str, default: &Option<String>) -> Result<PathBuf, Error> {
    let path_str = arg_or_default(s, default)?;
    Ok(PathBuf::from(&path_str))
}

/// Parse gas limit for transaction execution
pub fn parse_gas_or_default(s: &str, default: &Option<String>) -> Result<u64, Error> {
    let gas_str = arg_or_default(s, default)?;
    let gas = Vec::from_hex(to_even_str(trim_hex(&gas_str)))?;
    Ok(to_u64(&gas))
}

/// Parse gas limit for transaction execution
pub fn parse_gas_price_or_default(s: &str, default: &Option<String>) -> Result<[u8; 32], Error> {
    let gp_str = arg_or_default(s, default)?;
    hex_to_32bytes(&gp_str)
}

/// Request passphrase
pub fn request_passphrase() -> Result<String, Error> {
    println!("Enter passphrase: ");
    let passphrase = read!("{}\n");

    Ok(passphrase)
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

    /// Get nonce for address from remote node
    ///
    /// # Arguments:
    ///
    /// * addr - target address
    ///
    pub fn get_nonce(&self, addr: &Address) -> Result<u64, Error> {
        match self.connector {
            Some(ref conn) => {
                let data = vec![
                    Value::String(addr.to_string()),
                    Value::String("latest".to_string()),
                ];
                let params = Params::Array(data);

                let val = conn.send_post(
                    &MethodParams(ClientMethod::EthGetTxCount, &params),
                )?;
                match val.as_str() {
                    Some(s) => Ok(u64::from_str_radix(trim_hex(s), 16)?),
                    None => Err(Error::ExecError("Can't parse tx count".to_string())),
                }
            }
            None => {
                let nonce = parse_nonce(&self.args.flag_nonce)?;
                Ok(nonce)
            }
        }
    }


    /// Send signed raw transaction to the remote client
    ///
    /// # Arguments:
    ///
    /// * raw - signed tx
    ///
    /// # Return:
    ///
    /// * String - transaction hash
    ///
    pub fn send_transaction(&self, raw: Vec<u8>) -> Result<String, Error> {
        match self.connector {
            Some(ref conn) => {
                let data = vec![Value::String(format!("0x{}", raw.to_hex()))];
                let params = Params::Array(data);
                conn.send_post(&MethodParams(ClientMethod::EthSendRawTransaction, &params))
                    .and_then(|v| match v.as_str() {
                        Some(str) => Ok(str.to_string()),
                        None => Err(Error::ExecError("Can't parse tx hash".to_string())),
                    })
            }
            None => Err(Error::ExecError("Can't connect to client".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_private_key() {}

    #[test]
    fn should_parse_address() {
        let addr = Address::from_str("0xc0de379b51d582e1600c76dd1efee8ed024b844a").unwrap();
        let addr_parsed = parse_address("0xc0de379b51d582e1600c76dd1efee8ed024b844a").unwrap();
        assert_eq!(addr, addr_parsed);

        let addr_parsed = parse_address("c0de379b51d582e1600c76dd1efee8ed024b844a").unwrap();
        assert_eq!(addr, addr_parsed);

        let addr_parsed = parse_address("0xc0de379b51d5________0c76dd1efee8ed024b844a");
        assert!(addr_parsed.is_err());
    }

    #[test]
    fn should_parse_nonce() {}

    #[test]
    fn should_parse_value() {}

    #[test]
    fn should_parse_gas() {}

    #[test]
    fn should_parse_gas_price() {}

    #[test]
    fn should_convert_hex_to_32bytes() {}

    #[test]
    fn should_convert_arg_to_opt() {}
}
