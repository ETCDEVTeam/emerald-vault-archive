//! # Errors for command executor

use std::{error, fmt, io, str, string};
use emerald::storage::KeyStorageError;
use emerald::{self, keystore};
use std::net::AddrParseError;
use rustc_serialize::json;
use reqwest;
use std::num;

///
#[derive(Debug)]
pub enum Error {
    /// Command execution error
    ExecError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<KeyStorageError> for Error {
    fn from(err: KeyStorageError) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<string::ParseError> for Error {
    fn from(err: string::ParseError) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<emerald::Error> for Error {
    fn from(err: emerald::Error) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Self {
        Error::ExecError(format!("Can't parse host/port args: {}", err.to_string()))
    }
}

impl From<keystore::Error> for Error {
    fn from(err: keystore::Error) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<keystore::SerializeError> for Error {
    fn from(err: keystore::SerializeError) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<json::EncoderError> for Error {
    fn from(err: json::EncoderError) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ExecError(err.to_string())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ExecError(err.to_string())
    }
}

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
