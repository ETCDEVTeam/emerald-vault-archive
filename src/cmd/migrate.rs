//! # Storage migration related commands

use std::path::Path;
use super::ExecResult;

/// Migrate storage from old scheme to new
///
/// # Arguments:
///
/// *
///
// /home/k2/.emerald
// ├── [4.0K]  mainnet
// │   ├── [4.0K]  addressbook
// │   ├── [4.0K]  contracts
// │   │   ├── [1.8K]  0x0047201aed0b69875b24b614dda0270bcd9f11cc.json
// │   │   └── [  78]  0x085fb4f24031eaedbc2b611aa528f22343eb52db.json
// │   └── [4.0K]  keystore
// ├── [4.0K]  morden
// │   ├── [4.0K]  addressbook
// │   ├── [4.0K]  contracts
// │   └── [4.0K]  keystore
// └── [4.0K]  testnet
//     ├── [4.0K]  contracts
//     └── [4.0K]  keystore

pub fn migrate_cmd() -> ExecResult {
    println!("{}", Path::new("/home/.emerald/mainnet").exists());
    println!("{}", Path::new("/home/.emerald/morden").exists());
    println!("{}", Path::new("/home/.emerald/testnet").exists());

    Ok(())
}
