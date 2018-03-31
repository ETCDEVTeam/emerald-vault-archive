//! # Transaction related subcommands

use super::{rpc, ArgMatches, EnvVars, Error, ExecResult, KeyfileStorage, RpcConnector, Transaction};
use super::arg_handlers::*;
use std::io;
use std::io::Read;
use emerald::{trim_hex, Address};
use super::hex_to_32bytes;

/// Hide account from being listed
///
/// # Arguments:
///
/// * matches - arguments supplied from command-line
/// * storage - `Keyfile` storage
/// * sec_level - key derivation depth
///
pub fn transaction_cmd(
    matches: &ArgMatches,
    storage: &Box<KeyfileStorage>,
    env: &EnvVars,
) -> ExecResult {
    match matches.subcommand() {
        ("new", Some(sub_m)) => new(sub_m, env, storage),
        ("send", Some(sub_m)) => send(sub_m),
        _ => Err(Error::ExecError(
            "Invalid transaction subcommand. Use `emerald transaction -h` for help".to_string(),
        )),
    }
}

/// Create new transaction
///
///  # Arguments:
///
///  * matches -
///  * env -
///  * storage -
///  * rpc -
///
fn new(matches: &ArgMatches, env: &EnvVars, storage: &Box<KeyfileStorage>) -> ExecResult {
    let (_, kf) = get_address(matches).and_then(|from| storage.search_by_address(&from))?;
    let pk = request_passphrase().and_then(|pass| kf.decrypt_key(&pass))?;
    let signed = build_tx(matches, env).and_then(|tr| sign_tx(&tr, pk))?;

    println!("{}", signed.to_hex());

    Ok(())
}

/// Send transaction into network through provided node
///
///  # Arguments:
///
///  * matches -
///  * rpc -
///
fn send(matches: &ArgMatches, raw: &[u8]) -> ExecResult {
    let mut tx = String::new();

    let tx = match matches.value_of("signed-tx") {
        Some(t) => t,
        None =>  {
            let mut tx = String::new();
            io::stdin().read_to_string(&mut tx)?;
            tx
        }
    };

    match get_upstream(matches) {
        Ok(rpc) => {
            let tx_hash = rpc::send_transaction(conn, tx)?;
            println!("Tx hash: ");
            println!("{}", tx_hash);
            Ok(())
        }
        Err(err) => Err(Error::ExecError(format!(
            "Can't connect to node: {}",
            err.to_string()
        ))),
    }
}

/// Build transaction for provided arguments
/// If argument missing, try to use envirment vars
/// or request value through RPC
///
///  # Arguments:
///
///  * matches -
///  * env -
///  * rpc -
///
fn build_tx(matches: &ArgMatches, env: &EnvVars) -> Result<Transaction, Error> {
    let from = get_address(matches)?;
    let get_upstream = get_upstream(matches);

    let gas_price = matches
        .value_of("gas-price")
        .or(env.emerald_gas_price.as_ref().map(String::as_str))
        .or_else(get_upstream.and_then(|rpc| Some(rpc::get_gas_price(rpc)?)))
        .and_then(|s| Some(hex_to_32bytes(trim_hex(s))?))
        .expect("Required gas price");

    let gas = matches
        .value_of("gas")
        .or(env.emerald_gas.as_ref().map(String::as_str))
        .or_else(get_upstream.and_then(|rpc| Some(rpc::get_gas(rpc)?)))
        .and_then(|s| Some(u64::from_str_radix(trim_hex(s), 16)?))
        .expect("Required amount of gas");

    let nonce = matches
        .value_of("nonce")
        .or_else(get_upstream.and_then(|rpc| Some(rpc::get_nonce(rpc, &from)?)))
        .and_then(|s| Some(u64::from_str_radix(trim_hex(s), 16)?))
        .expect("Required nonce value for sender");

    let to = matches.value_of("to").and_then(|| parse_address);

    let value = matches
        .value_of("value")
        .and_then(|v| Some(hex_to_32bytes(v)?))
        .expect("Required value to send");

    let data = matches
        .value_of("value")
        .and_then(Vec::from_hex)
        .unwrap_or(vec![]);

    Ok(Transaction {
        nonce,
        gas_price,
        gas_limit,
        to,
        value,
        data,
    })
}

/// Sign transaction with private key
///
///  # Arguments:
///
///  * matches -
///  * env -
///  * rpc -
///
fn sign_tx(tr: &Transaction, pk: PrivateKey) -> Result<Vec<u8>, Error> {
    if let Some(chain_id) = to_chain_id(&self.chain) {
        let raw = tr.to_signed_raw(pk, chain_id)?;
        Ok(raw)
    } else {
        Err(Error::ExecError("Invalid chain name".to_string()))
    }
}

/// Parse nonce value,
/// or try to request from network node
///
///  # Arguments:
///
///  * matches -
///  * env -
///  * rpc -
///
fn parse_nonce(s: &str, rpc: &Option<RpcConnector>, addr: Option<Address>) -> Result<u64, Error> {
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

///// Parse gas limit for transaction execution,
/////  or try to request from network node
/////
/////  # Arguments:
/////
/////  * matches -
/////  * env -
/////  * rpc -
/////
//fn parse_gas_or_default(
//    s: &str,
//    default: &Option<String>,
//    rpc: &Option<RpcConnector>,
//) -> Result<u64, Error> {
//    match arg_or_default(s, default).and_then(|s| parse_arg(&s)) {
//        Ok(gas) => Ok(u64::from_str_radix(&gas, 16)?),
//        Err(e) => match *rpc {
//            Some(ref conn) => Ok(rpc::get_gas(conn)?),
//            None => Err(e),
//        },
//    }
//}
//
///// Parse gas price for transaction execution,
///// or try to request from network node
/////
/////  # Arguments:
/////
/////  * matches -
/////  * env -
/////  * rpc -
/////
//fn parse_gas_price_or_default(
//    s: &str,
//    default: &Option<String>,
//    rpc: &Option<RpcConnector>,
//) -> Result<[u8; 32], Error> {
//    match arg_or_default(s, default).and_then(|s| parse_arg(&s)) {
//        Ok(s) => hex_to_32bytes(&s),
//        Err(e) => match *rpc {
//            Some(ref conn) => Ok(rpc::get_gas_price(conn)?),
//            None => Err(e),
//        },
//    }
//}
