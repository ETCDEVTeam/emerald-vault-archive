//! # Errors for command executor

use std::{error, fmt, io, str};
use emerald::storage::KeyStorageError;

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
