use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use anchor_lang::prelude::*;
// use std::str::FromStr;
use std::{rc::Rc, str::FromStr};

#[test]
// fn test_initialize() {
//     let program_id = "Eub9MuruafRZbPzcmYzjNMGYtCnGm1GknoGJHfx2dfSy";
//     let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
//     let payer = read_keypair_file(&anchor_wallet).unwrap();

//     let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
//     let program_id = Pubkey::from_str(program_id).unwrap();
//     let program = client.program(program_id).unwrap();

//     let tx = program
//         .request()
//         .accounts(hello_world::accounts::Initialize {})
//         .args(hello_world::instruction::Initialize {})
//         .send()
//         .expect("");

//     println!("Your transaction signature {}", tx);
// }

fn test_hello_world() {
    println!("Hello, world! From Solana smart contract");
    // Program ID of the deployed HelloWorld program
    let program_id = "Eub9MuruafRZbPzcmYzjNMGYtCnGm1GknoGJHfx2dfSy";

    // Set up wallet path for the payer
    let anchor_wallet =
        std::env::var("ANCHOR_WALLET").expect("ANCHOR_WALLET environment variable not set");
    let payer = Rc::new(read_keypair_file(&anchor_wallet).expect("Failed to read keypair file"));

    // Initialize client
    let client = Client::new_with_options(
        Cluster::Localnet,
        payer.clone(),
        CommitmentConfig::confirmed(),
    );
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // Send the transaction to call the "helloWorld" method
    let tx = program
        .request()
        .accounts(hello_world::accounts::Initialize {}) // Adjust based on account layout for your program
        .args(hello_world::instruction::HelloWorld {}) // Use the appropriate instruction generated by Anchor
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
    msg!("Hello, world! From Solana smart contract");
}
