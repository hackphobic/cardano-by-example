use whisky::Wallet;

use std::path::{Path, PathBuf};
use std::fs;
use std::str::from_utf8;
use std::error::Error;
use dirs::home_dir;

const WALLET_DIR: &str = ".cardano";
const KEYSTORE_FILENAME: &str = "phrase";


fn main() {
    let wallet_path = match default_wallet_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not get the path to home directory");
            return;
        }
    };

    let phrase_bytes = match fs::read(wallet_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Error: Could not read the phrase file");
            return;
        }
    };

    let phrase = match std::str::from_utf8(phrase_bytes.as_slice()) {
        Ok(phrase) => phrase,
        Err(_) => {
            println!("Error: could not convert phrase bytes to string");
            return;
        }
    };

    println!("{}", phrase);

    let mut wallet = match Wallet::new_mnemonic(phrase) {
        Ok(w) => w,
        Err(_) => {
            println!("Error: could not create wallet from mnemonic");
            return;
        }
    };

    let payment_account = match wallet.payment_account(0, 0) {
        Ok(account) => account,
        Err(_) => {
            println!("Error: could not get account @0,0");
            return;
        }
    };
    
    println!("{}", payment_account.account.public_key.to_bech32());
}


fn default_wallet_path() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(path.join(WALLET_DIR).join(KEYSTORE_FILENAME)),
        None => None,
    }
}
