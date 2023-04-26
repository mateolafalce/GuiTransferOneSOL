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

// Deriving the Debug, Deserialize, and Serialize traits for the Wallet struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub wallet: Vec<u8> // Defining a Wallet struct with a public wallet field of type Vec<u8>
}

pub fn store_wallet(
    new_wallet: Vec<u8>
) -> Result<(), Error> { // Defining a function called store_wallet that takes a Vec<u8> and returns a Result<(), Error>
    let mut wallet: Wallet = from_str(&read_to_string("src/wallet.json").unwrap()).unwrap(); // Reading the contents of "src/wallet.json" into a Wallet struct
    wallet.wallet = new_wallet; // Setting the wallet field of the Wallet struct to the new_wallet value
    write("src/wallet.json", to_string(&wallet).unwrap()).unwrap(); // Writing the updated Wallet struct to "src/wallet.json"
    Ok(())
}
