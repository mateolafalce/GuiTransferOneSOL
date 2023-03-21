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
use crate::functions::Wallet;

pub fn clear() -> Result<(), Error> {
    let mut wallet: Wallet = from_str(&read_to_string("src/wallet.json").unwrap()).unwrap();
    wallet.wallet = vec![];
    write("src/wallet.json", to_string(&wallet).unwrap()).unwrap();
    Ok(())
}
