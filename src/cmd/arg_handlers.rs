//! # Helpers for command execution

use super::Error;
use super::{align_bytes, to_arr, to_even_str, trim_hex, Address, CmdExecutor, KeyFile, PrivateKey};
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::env;
use hex::FromHex;
use rustc_serialize::json;
use std::fs;
use std::io::Write;
use rpassword;
use emerald::Transaction;
use rpc::{self, RpcConnector};
use hyper::Url;
use hyper::client::IntoUrl;
use std::net::SocketAddr;

/// Environment variables used to change default variables
#[derive(Default, Debug)]
pub struct EnvVars<'a> {
    pub emerald_base_path: Option<&'a str>,
    pub emerald_host: Option<&'a str>,
    pub emerald_port: Option<&'a str>,
    pub emerald_chain: Option<&'a str>,
    pub emerald_chain_id: Option<&'a str>,
    pub emerald_gas: Option<&'a str>,
    pub emerald_gas_price: Option<&'a str>,
    pub emerald_security_level: Option<&'a str>,
    pub emerald_node: Option<&'a str>,
}

impl<'a> EnvVars<'a> {
    /// Collect environment variables to overwrite default values
    pub fn parse() -> EnvVars<'a> {
        let mut vars = EnvVars::default();
        for (key, value) in env::vars() {
            match key.as_ref() {
                "EMERALD_BASE_PATH" => vars.emerald_base_path = Some(&value),
                "EMERALD_HOST" => vars.emerald_host = Some(&value),
                "EMERALD_PORT" => vars.emerald_port = Some(&value),
                "EMERALD_CHAIN" => vars.emerald_chain = Some(&value),
                "EMERALD_CHAIN_ID" => vars.emerald_chain_id = Some(&value),
                "EMERALD_GAS" => vars.emerald_gas = Some(&value),
                "EMERALD_GAS_PRICE" => vars.emerald_gas_price = Some(&value),
                "EMERALD_SECURITY_LEVEL" => vars.emerald_security_level = Some(&value),
                "EMERALD_NODE" => vars.emerald_node = Some(&value),
                _ => (),
            }
        }
        vars
    }
}

/// Try to parse argument
/// If no `arg` was supplied, try to use environment variable
///
/// # Arguments:
///
/// * arg - provided argument
/// * env - optional environment variable
///
pub fn arg_or_default(arg: &str, env: &Option<String>) -> Result<String, Error> {
    let val = arg.parse::<String>()?;

    if val.is_empty() {
        env.clone()
            .ok_or_else(|| Error::ExecError("Missed arguments".to_string()))
    } else {
        Ok(val)
    }
}

/// Try to parse argument into optional string
///
/// # Arguments:
///
/// * arg - provided argument
///
pub fn arg_to_opt(arg: &str) -> Result<Option<String>, Error> {
    let str = arg.parse::<String>()?;
    if str.is_empty() {
        Ok(None)
    } else {
        Ok(Some(str))
    }
}

/// Parse raw hex string arguments from user
fn parse_arg(raw: &str) -> Result<String, Error> {
    let s = raw.parse::<String>()
        .and_then(|s| Ok(to_even_str(trim_hex(&s))))?;

    if s.is_empty() {
        Err(Error::ExecError(
            "Invalid parameter: empty string".to_string(),
        ))
    } else {
        Ok(s)
    }
}

/// Converts hex string to 32 bytes array
/// Aligns original `hex` to fit 32 bytes
pub fn hex_to_32bytes(hex: &str) -> Result<[u8; 32], Error> {
    if hex.is_empty() {
        return Err(Error::ExecError(
            "Invalid parameter: empty string".to_string(),
        ));
    }

    let bytes = Vec::from_hex(hex)?;
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
    let value_str = parse_arg(s)?;
    hex_to_32bytes(&value_str)
}

/// Parse transaction data
pub fn parse_data(s: &str) -> Result<Vec<u8>, Error> {
    let data = match s.parse::<String>()
        .and_then(|d| Ok(to_even_str(trim_hex(&d))))
    {
        Ok(str) => Vec::from_hex(&str)?,
        Err(_) => vec![],
    };

    Ok(data)
}

/// Parse path for accounts import/export
pub fn parse_path_or_default(s: &str, default: &Option<String>) -> Result<PathBuf, Error> {
    let path_str = arg_or_default(s, default)?;
    Ok(PathBuf::from(&path_str))
}

/// Parse nonce value,
/// or try to request from network node
pub fn parse_nonce(
    s: &str,
    rpc: &Option<RpcConnector>,
    addr: Option<Address>,
) -> Result<u64, Error> {
    match parse_arg(s) {
        Ok(nonce) => Ok(u64::from_str_radix(&nonce, 16)?),
        Err(e) => match *rpc {
            Some(ref conn) => {
                if let Some(a) = addr {
                    Ok(rpc::get_nonce(conn, &a)?)
                } else {
                    Err(e)
                }
            }
            None => Err(e),
        },
    }
}

/// Parse gas limit for transaction execution,
///  or try to request from network node
pub fn parse_gas_or_default(
    s: &str,
    default: &Option<String>,
    rpc: &Option<RpcConnector>,
) -> Result<u64, Error> {
    match arg_or_default(s, default).and_then(|s| parse_arg(&s)) {
        Ok(gas) => Ok(u64::from_str_radix(&gas, 16)?),
        Err(e) => match *rpc {
            Some(ref conn) => Ok(rpc::get_gas(conn)?),
            None => Err(e),
        },
    }
}

/// Parse gas price for transaction execution,
/// or try to request from network node
pub fn parse_gas_price_or_default(
    s: &str,
    default: &Option<String>,
    rpc: &Option<RpcConnector>,
) -> Result<[u8; 32], Error> {
    match arg_or_default(s, default).and_then(|s| parse_arg(&s)) {
        Ok(s) => hex_to_32bytes(&s),
        Err(e) => match *rpc {
            Some(ref conn) => Ok(rpc::get_gas_price(conn)?),
            None => Err(e),
        },
    }
}

/// Parse URL for ethereum node
pub fn parse_url(s: &str) -> Result<Url, Error> {
    let addr = Url::parse(s).map_err(Error::from)?;
    Ok(addr)
}

/// Parse socket address for ethereum node
pub fn parse_socket(s: &str) -> Result<Url, Error> {
    let addr = s.parse::<SocketAddr>()
        .map_err(Error::from)
        .and_then(|a| format!("https://{}", a).into_url().map_err(Error::from))?;

    Ok(addr)
}

/// Request passphrase
pub fn request_passphrase() -> Result<String, Error> {
    println!("Enter passphrase: ");
    let passphrase = rpassword::read_password().unwrap();

    Ok(passphrase)
}

//impl<'a> CmdExecutor<'a> {
//    /// Import Keyfile into storage
//    pub fn import_keyfile<P: AsRef<Path>>(&self, path: P, force_mode: bool) -> Result<(), Error> {
//        let mut json = String::new();
//        File::open(path).and_then(|mut f| f.read_to_string(&mut json))?;
//
//        let kf = KeyFile::decode(&json)?;
//        let st = self.storage_ctrl.get_keystore(&self.chain)?;
//
//        match st.is_addr_exist(&kf.address) {
//            Ok(_) => {
//                if force_mode {
//                    st.put(&kf)?;
//                }
//            }
//            Err(_) => st.put(&kf)?,
//        }
//
//        Ok(())
//    }
//
//    /// Export Keyfile for selected address
//    /// into into specified file
//    ///
//    /// # Arguments:
//    ///
//    /// * addr - target addr
//    /// * path - target file path
//    ///
//    pub fn export_keyfile(&self, addr: &Address, path: &Path) -> Result<(), Error> {
//        let st = self.storage_ctrl.get_keystore(&self.chain)?;
//        let (info, kf) = st.search_by_address(addr)?;
//
//        let mut p = PathBuf::from(path);
//        p.push(&info.filename);
//
//        let json = json::encode(&kf).and_then(|s| Ok(s.into_bytes()))?;
//        let mut f = fs::File::create(p)?;
//        f.write_all(&json)?;
//
//        Ok(())
//    }
//
//    /// Build transaction for provided arguments
//    pub fn build_tx(&self) -> Result<Transaction, Error> {
//        let from = parse_address(&self.args.arg_from)?;
//        let tr = Transaction {
//            nonce: parse_nonce(&self.args.flag_nonce, &self.connector, Some(from))?,
//            gas_price: parse_gas_price_or_default(
//                &self.args.flag_gas_price,
//                &self.vars.emerald_gas_price,
//                &self.connector,
//            )?,
//            gas_limit: parse_gas_or_default(
//                &self.args.flag_gas,
//                &self.vars.emerald_gas,
//                &self.connector,
//            )?,
//            to: match parse_address(&self.args.arg_to) {
//                Ok(a) => Some(a),
//                Err(_) => None,
//            },
//            value: parse_value(&self.args.arg_value)?,
//            data: parse_data(&self.args.flag_data)?,
//        };
//
//        Ok(tr)
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_hex_to_32bytes() {
        assert_eq!(
            hex_to_32bytes("fa384e6fe915747cd13faa1022044b0def5e6bec4238bec53166487a5cca569f",)
                .unwrap(),
            [
                0xfa, 0x38, 0x4e, 0x6f, 0xe9, 0x15, 0x74, 0x7c, 0xd1, 0x3f, 0xaa, 0x10, 0x22, 0x04,
                0x4b, 0x0d, 0xef, 0x5e, 0x6b, 0xec, 0x42, 0x38, 0xbe, 0xc5, 0x31, 0x66, 0x48, 0x7a,
                0x5c, 0xca, 0x56, 0x9f,
            ]
        );
        assert_eq!(hex_to_32bytes("00").unwrap(), [0u8; 32]);
        assert_eq!(hex_to_32bytes("0000").unwrap(), [0u8; 32]);
        assert!(hex_to_32bytes("00_10000").is_err());
        assert!(hex_to_32bytes("01000z").is_err());
        assert!(hex_to_32bytes("").is_err());
    }

    #[test]
    fn should_parse_arg() {
        assert_eq!(parse_arg("0x1000").unwrap(), "1000");
        assert_eq!(parse_arg("0x100").unwrap(), "0100");
        assert_eq!(parse_arg("0x10000").unwrap(), "010000");
        assert!(parse_arg("0x").is_err());
        assert!(parse_arg("").is_err());
    }

    #[test]
    fn should_convert_arg_to_opt() {
        assert_eq!(arg_to_opt("").unwrap(), None);
        assert_eq!(arg_to_opt("test").unwrap(), Some("test".to_string()));
    }

    #[test]
    fn should_parse_private_key() {
        let pk = PrivateKey::try_from(&[0u8; 32]).unwrap();
        assert_eq!(
            pk,
            parse_pk("0x0000000000000000000000000000000000000000000000000000000000000000",)
                .unwrap()
        );
    }

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
    fn should_parse_nonce() {
        assert_eq!(parse_nonce("0x1000", &None, None).unwrap(), 4096);
        assert_eq!(parse_nonce("0x01000", &None, None).unwrap(), 4096);
        assert_eq!(parse_nonce("0x100", &None, None).unwrap(), 256);
        assert_eq!(parse_nonce("0x0100", &None, None).unwrap(), 256);
        assert!(parse_nonce("", &None, None).is_err());
    }

    #[test]
    fn should_parse_value() {
        assert_eq!(parse_value("0x00").unwrap(), [0u8; 32]);
        assert_eq!(parse_value("000").unwrap(), [0u8; 32]);
        assert!(parse_value("00_10000").is_err());
        assert!(parse_value("01000z").is_err());
        assert!(parse_value("").is_err());
    }

    #[test]
    fn should_parse_data() {
        assert_eq!(parse_data("0x00").unwrap(), vec![0]);
        assert_eq!(parse_data("000").unwrap(), vec![0, 0]);
        assert_eq!(parse_data("").unwrap(), Vec::new() as Vec<u8>);
        assert!(parse_data("00_10000").is_err());
        assert!(parse_data("01000z").is_err());
    }

    #[test]
    fn should_parse_gas() {
        assert_eq!(parse_gas_or_default("0x000", &None, &None).unwrap(), 0);
        assert_eq!(
            parse_gas_or_default("", &Some("0x000".to_string()), &None).unwrap(),
            0
        );
        assert!(parse_gas_or_default("", &None, &None).is_err());
    }

    #[test]
    fn should_parse_gas_price() {
        assert_eq!(
            parse_gas_price_or_default("0x000", &None, &None).unwrap(),
            [0u8; 32]
        );
        assert_eq!(
            parse_gas_price_or_default("", &Some("0x000".to_string()), &None).unwrap(),
            [0u8; 32]
        );
        assert!(parse_gas_price_or_default("", &None, &None).is_err());
    }

    #[test]
    fn should_parse_socket_addr() {
        assert_eq!(
            parse_socket("127.0.0.1:8545").unwrap(),
            Url::parse("https://127.0.0.1:8545").unwrap()
        );

        assert!(parse_socket(";akjf.com").is_err());
        assert!(parse_socket("https://127.0.0.1:8545").is_err());
    }

    #[test]
    fn should_parse_url_name() {
        assert_eq!(
            parse_url("https://www.gastracker.io").unwrap(),
            Url::parse("https://www.gastracker.io").unwrap()
        );

        assert!(parse_url("127.0.0.1:8545").is_err());
        assert!(parse_url("12344.com").is_err());
    }
}
