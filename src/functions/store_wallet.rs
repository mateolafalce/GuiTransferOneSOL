use serde_json::{
    from_str,
    to_string
};
use std::{
    fs::{
        write,
        read_to_string
    },
    io::Error,
};
use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub wallet: Vec<u8>
}

pub fn store_wallet(new_wallet: Vec<u8>) -> Result<(), Error> {
    let mut wallet: Wallet = from_str(&read_to_string("src/wallet.json").unwrap()).unwrap();
    wallet.wallet = new_wallet;
    write("src/wallet.json", to_string(&wallet).unwrap()).unwrap();
    Ok(())
}
