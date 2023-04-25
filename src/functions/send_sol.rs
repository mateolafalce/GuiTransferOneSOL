use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{
            Keypair,
            Signature
        },
        signer::Signer,
    },
    Client, Cluster, Program,
};
use std::{
    io::Error,
    fs::read_to_string,
    str::FromStr,
    rc::Rc
};
use crate::functions::Wallet; // Import Wallet struct from functions module
use serde_json::from_str;

pub fn send_sol(
    message:String
) -> Result<(), Error> {
    let contents: String = read_to_string("src/wallet.json").unwrap(); // Read wallet file into a string
    let wallet: Wallet = from_str(&contents).unwrap(); // Deserialize wallet string into Wallet struct
    let payer: Keypair = Keypair::from_bytes(&wallet.wallet).expect("Requires a keypair"); // Create a Keypair from wallet bytes
    let client: Client = Client::new(Cluster::Devnet, Rc::new(payer)); // Create a client with payer keypair
    let program: Program =
        client.program(
            Pubkey::from_str(
                &"97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb".to_string() // Convert string to Pubkey
            ).unwrap()
        ); // Load program with given pubkey
    transfer(program, message).expect("Send"); // Call transfer function with program and message
    Ok(()) // Return Ok result
}


pub fn transfer(
    program: Program,
    msg: String
) -> Result<(), Error> {
    let luck_addres: Keypair = Keypair::new(); // Create a new keypair to hold the destination address
    let signature: Signature = program // Send a transaction to the program with the specified accounts and arguments
        .request()
        .accounts(transfer_one_sol::accounts::Transaction {
            from: program.payer(), // Set the account to send the funds from as the payer of the program
            to: luck_addres.pubkey(), // Set the destination address to the new keypair we created earlier
            system_program: system_program::ID, // Set the system program to use for this transaction
        })
        .args(transfer_one_sol::instruction::SendOneSol { msg }) // Set the argument to the `SendOneSol` instruction
        .send().unwrap(); 
    println!("{}", signature); // Print the signature of the transaction to the console
    Ok(()) // Return a successful result
}
