//! # JSON RPC module

mod http;
mod serialize;

pub use self::http::{RpcConnector, MethodParams, ClientMethod};
use jsonrpc_core::{Value, Params};
use cmd::Error;
use hex::ToHex;
use emerald::{Address, trim_hex};

/// Get nonce for address from remote node
///
/// # Arguments:
///
/// * addr - target address
///
pub fn get_nonce(rpc: &RpcConnector, addr: &Address) -> Result<u64, Error> {
    let data = vec![
        Value::String(addr.to_string()),
        Value::String("latest".to_string()),
    ];
    let params = Params::Array(data);
    let val = rpc.send_post(
        &MethodParams(ClientMethod::EthGetTxCount, &params),
    )?;

    match val.as_str() {
        Some(s) => Ok(u64::from_str_radix(trim_hex(s), 16)?),
        None => Err(Error::ExecError("Can't parse tx count".to_string())),
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
pub fn send_transaction(rpc: &RpcConnector, raw: Vec<u8>) -> Result<String, Error> {
    let data = vec![Value::String(format!("0x{}", raw.to_hex()))];
    let params = Params::Array(data);
    rpc.send_post(&MethodParams(ClientMethod::EthSendRawTransaction, &params))
        .and_then(|v| match v.as_str() {
            Some(str) => Ok(str.to_string()),
            None => Err(Error::ExecError("Can't parse tx hash".to_string())),
        })
}

/// Get balance for selected account
///
/// # Arguments:
///
/// * addr - target account
///
/// # Return:
///
/// * String - latest balance
///
pub fn get_balance(rpc: &RpcConnector, addr: &Address) -> Result<String, Error> {
    let data = vec![
        Value::String(addr.to_string()),
        Value::String("latest".to_string()),
    ];

    let params = Params::Array(data);
    rpc.send_post(&MethodParams(ClientMethod::EthGetBalance, &params))
        .and_then(|v| match v.as_str() {
            Some(str) => Ok(str.to_string()),
            None => Err(Error::ExecError(format!("Can't get balance for {}", addr))),
        })
}
