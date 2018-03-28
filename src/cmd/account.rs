use super::{ExecResult, Error, KeyfileStorage};
use clap::ArgMatches;


///
pub fn account_cmd(matches: &ArgMatches, storage_ctrl: &Box<KeyfileStorage>) -> ExecResult {
    match matches.subcommand() {
        ("list", Some(sub_m))  => list(sub_m, storage_ctrl.clone()),
//        ("new", Some(sub_m))  => new(),
//        ("hide", Some(sub_m))  => hide(),
//        ("unhide", Some(sub_m))  => unhide(),
//        ("strip", Some(sub_m))  => strip(),
//        ("import", Some(sub_m))  => import(),
//        ("export", Some(sub_m))  => export(),
//        ("update", Some(sub_m))  => update(),
        _                        => Err(Error::ExecError("Invalid account command. Use `emerald account -h` for help".to_string())),
    }
}

/// List all accounts
fn list(matches: &ArgMatches, keystore: &Box<KeyfileStorage>) -> ExecResult {
    let accounts_info = keystore.list_accounts(matches.is_present("show-hidden"))?;

    println!("{0: <45} {1: <45} ", "ADDRESS", "NAME");
    for info in accounts_info {
        println!("{0: <45} {1: <45} ", &info.address, &info.name);
    }

    Ok(())
}
//
///// Creates new account
//fn new(matches: &ArgMatches) -> ExecResult {
//    println!("! Warning: passphrase can't be restored. Don't forget it !");
//    let passphrase = request_passphrase()?;
//    let name = arg_to_opt(matches: &ArgMatches.args.flag_name)?;
//    let desc = arg_to_opt(matches: &ArgMatches.args.flag_description)?;
//
//    let kf = if self.args.flag_raw {
//        let pk = parse_pk(matches: &ArgMatches.args.arg_key)?;
//        let mut kf = KeyFile::new(&passphrase, matches: &ArgMatches.sec_level, name, desc)?;
//        kf.encrypt_key(pk, &passphrase);
//        kf
//    } else {
//        KeyFile::new(&passphrase, matches: &ArgMatches.sec_level, name, desc)?
//    };
//
//    let st = self.storage_ctrl.get_keystore(matches: &ArgMatches.chain)?;
//    st.put(&kf)?;
//    println!("Created new account: {}", &kf.address.to_string());
//
//    Ok(())
//}
//
///// Hide account from being listed
//fn hide(matches: &ArgMatches) -> ExecResult {
//    let address = parse_address(matches: &ArgMatches.args.arg_address)?;
//    let st = self.storage_ctrl.get_keystore(matches: &ArgMatches.chain)?;
//    st.hide(&address)?;
//
//    Ok(())
//}
//
///// Unhide account from being listed
//fn unhide(matches: &ArgMatches) -> ExecResult {
//    let address = parse_address(matches: &ArgMatches.args.arg_address)?;
//    let st = self.storage_ctrl.get_keystore(matches: &ArgMatches.chain)?;
//    st.unhide(&address)?;
//
//    Ok(())
//}
//
///// Extract private key from a keyfile
//fn strip(matches: &ArgMatches) -> ExecResult {
//    let address = parse_address(matches: &ArgMatches.args.arg_address)?;
//    let st = self.storage_ctrl.get_keystore(matches: &ArgMatches.chain)?;
//
//    let (_, kf) = st.search_by_address(&address)?;
//    let passphrase = request_passphrase()?;
//    let pk = kf.decrypt_key(&passphrase)?;
//
//    println!("Private key: {}", &pk.to_string());
//
//    Ok(())
//}
//
///// Export accounts
//fn export(matches: &ArgMatches) -> ExecResult {
//    let path = parse_path_or_default(matches: &ArgMatches.args.arg_path, matches: &ArgMatches.vars.emerald_base_path)?;
//
//    if self.args.flag_all {
//        if !path.is_dir() {
//            return Err(Error::ExecError(
//                "`export`: invalid args. Use `-h` for help.".to_string(),
//            ));
//        }
//
//        let st = self.storage_ctrl.get_keystore(matches: &ArgMatches.chain)?;
//        let accounts_info = st.list_accounts(true)?;
//        for info in accounts_info {
//            let addr = Address::from_str(&info.address)?;
//            self.export_keyfile(&addr, &path)?
//        }
//    } else {
//        let addr = parse_address(matches: &ArgMatches.args.arg_address)?;
//        self.export_keyfile(&addr, &path)?
//    }
//
//    Ok(())
//}
//
///// Import accounts
//fn import(matches: &ArgMatches) -> ExecResult {
//    let path = parse_path_or_default(matches: &ArgMatches.args.arg_path, matches: &ArgMatches.vars.emerald_base_path)?;
//    let mut counter = 0;
//
//    if path.is_file() {
//        self.import_keyfile(path, self.args.flag_force)?;
//        counter += 1;
//    } else {
//        let entries = fs::read_dir(&path)?;
//        for entry in entries {
//            let path = entry?.path();
//            if path.is_dir() {
//                continue;
//            }
//            self.import_keyfile(path, self.args.flag_force)?;
//            counter += 1;
//        }
//    }
//
//    println!("Imported accounts: {}", counter);
//
//    Ok(())
//}
//
///// Update `name` and `description` for existing account
//fn update(matches: &ArgMatches) -> ExecResult {
//    let address = parse_address(matches: &ArgMatches.args.arg_address)?;
//    let name = arg_to_opt(matches: &ArgMatches.args.flag_name)?;
//    let desc = arg_to_opt(matches: &ArgMatches.args.flag_description)?;
//
//    let st = self.storage_ctrl.get_keystore(matches: &ArgMatches.chain)?;
//    st.update(&address, name, desc)?;
//
//    Ok(())
//}
//
