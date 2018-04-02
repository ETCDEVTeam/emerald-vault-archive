//! # Errors for command executor

use std::{error, fmt, io, str, string};
use emerald::storage::KeystoreError;
use emerald::{self, keystore};
use std::net::AddrParseError;
use rustc_serialize::json;
use reqwest;
use std::num;
use hex;
use url;

macro_rules! from_err {
    ($x: ty) => {
        impl From<$x> for Error {
            fn from(err: $x) -> Self {
                Error::ExecError(err.to_string())
            }
        }
    };
}

///
#[derive(Debug)]
pub enum Error {
    /// Command execution error
    ExecError(String),
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Self {
        Error::ExecError(format!("Can't parse host/port args: {}", err.to_string()))
    }
}

from_err!(io::Error);
from_err!(KeystoreError);
from_err!(string::ParseError);
from_err!(keystore::Error);
from_err!(keystore::SerializeError);
from_err!(json::EncoderError);
from_err!(reqwest::Error);
from_err!(num::ParseIntError);
from_err!(hex::FromHexError);
from_err!(emerald::Error);
from_err!(emerald::mnemonic::Error);
from_err!(url::ParseError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ExecError(ref str) => write!(f, "Command execution error: {}", str),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Command execution error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}
