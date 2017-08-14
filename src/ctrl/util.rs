//! # Helpers for command execution


use super::Error;
use super::{CmdExecutor, Address, PrivateKey};
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use std::fs::File;
use std::str::FromStr;


impl CmdExecutor {
    ///
    pub fn parse_address(&self) -> Result<Address, Error> {
        let addr_str = self.args.arg_address.parse::<String>()?;
        let add = Address::from_str(&addr_str)?;

        Ok(add)
    }

    ///
    pub fn parse_pk(&self) -> Result<PrivateKey, Error> {
        let pk_str = self.args.arg_path.parse::<String>()?;
        let pk = PrivateKey::from_str(&pk_str)?;

        Ok(pk)
    }

    ///
    pub fn parse_path(&self) -> Result<PathBuf, Error> {
        let pk_str = self.args.arg_path.parse::<String>()?;
        let pk = PathBuf::from(&pk_str);

        Ok(pk)
    }

    pub fn request_passphrase() -> Result<String, Error> {
        let mut out = io::stdout();
        out.write_all(b"Enter passphrase: \n")?;
        out.flush()?;

        let mut passphrase = String::new();
        io::stdin().read_line(&mut passphrase)?;

        Ok(passphrase)
    }

    pub fn file_content<P: AsRef<Path>>(path: P) -> Result<String, Error> {
        let mut text = String::new();

        File::open(path).and_then(
            |mut f| f.read_to_string(&mut text),
        )?;

        Ok(text)
    }
}
