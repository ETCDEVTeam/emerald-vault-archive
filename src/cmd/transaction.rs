//
//
/////
//fn transaction() -> ExecResult {
//    let st = self.storage_ctrl.get_keystore(&self.chain)?;
//    let from = parse_address(&self.args.arg_from)?;
//    let (_, kf) = st.search_by_address(&from)?;
//
//    let pass = request_passphrase()?;
//    let pk = kf.decrypt_key(&pass)?;
//
//    if self.args.flag_raw {
//        let mut raw_tx = String::new();
//        io::stdin().read_to_string(&mut raw_tx)?;
//        println!("raw tx {}", raw_tx);
//        Ok(())
//    } else {
//        let tr = self.build_tx()?;
//        let raw = self.sign_transaction(&tr, pk)?;
//
//        match self.connector {
//            Some(_) => self.send_transaction(&raw),
//            None => {
//                println!("Signed transaction: ");
//                println!("{}", raw.to_hex());
//                Ok(())
//            }
//        }
//    }
//}
//
///// Sign transaction with
//fn sign_transaction(&self, tr: &Transaction, pk: PrivateKey) -> Result<Vec<u8>, Error> {
//    if let Some(chain_id) = to_chain_id(&self.chain) {
//        let raw = tr.to_signed_raw(pk, chain_id)?;
//        Ok(raw)
//    } else {
//        Err(Error::ExecError("Invalid chain name".to_string()))
//    }
//}
//
///// Send transaction into network through provided node
//fn send_transaction(&self, raw: &[u8]) -> ExecResult {
//    match self.connector {
//        Some(ref conn) => {
//            let tx_hash = rpc::send_transaction(conn, raw)?;
//            println!("Tx hash: ");
//            println!("{}", tx_hash);
//            Ok(())
//        }
//
//        None => Err(Error::ExecError("Can't connect to node".to_string())),
//    }
//}
