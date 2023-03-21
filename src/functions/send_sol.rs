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
use crate::functions::Wallet;
use serde_json::from_str;

pub fn send_sol(message:String) -> Result<(), Error> {
    let contents: String = read_to_string("src/wallet.json").unwrap();
    let wallet: Wallet = from_str(&contents).unwrap();
    let payer: Keypair = Keypair::from_bytes(&wallet.wallet).expect("Requires a keypair");
    let client: Client = Client::new(Cluster::Devnet, Rc::new(payer));
    let program: Program =
        client.program(
            Pubkey::from_str(
                &"97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb".to_string()
            ).unwrap()
        );
    transfer(program, message).expect("Send");
    Ok(())
}

pub fn transfer(program: Program, msg: String) -> Result<(), Error> {
    let luck_addres: Keypair = Keypair::new();
    let signature: Signature = program
        .request()
        .accounts(transfer_one_sol::accounts::Transaction {
            from: program.payer(),
            to: luck_addres.pubkey(),
            system_program: system_program::ID,
        })
        .args(transfer_one_sol::instruction::SendOneSol { msg })
        .send().unwrap();
    println!("{}", signature);
    Ok(())
}
