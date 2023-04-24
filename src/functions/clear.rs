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
use crate::functions::Wallet; // Import Wallet struct from crate functions module

pub fn clear() -> Result<(), Error> { // Declare a function named clear that returns a Result type with empty tuple and Error type
    let mut wallet: Wallet = from_str(
        &read_to_string("src/wallet.json").unwrap()
    ).unwrap(); // Read wallet.json file and deserialize its content to a Wallet struct variable named wallet
    wallet.wallet = vec![]; // Assign an empty vector to wallet.wallet
    write(
        "src/wallet.json",
        to_string(&wallet
        ).unwrap())
    .unwrap(); // Serialize wallet struct to a string and write it to wallet.json file
    Ok(())
}
