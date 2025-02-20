use std::rc::Rc;
use std::str::FromStr;
use anchor_client::anchor_lang::prelude::*; use anchor_client::solana_sdk::signer::EncodableKey;
// ✅ Covers Pubkey
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use cnctd_studio_program::instruction::MintAlbum;
use cnctd_studio_program::requests::contract_request::{ContractRequest, Credits};

#[test]
fn test_mint_album() {
    let program_id = "4X4ghmVDL7B29vNns8qizXsMUcsE8TZi1EvPewLsPkrN"; 
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap(); 
    let payer = Keypair::read_from_file(&anchor_wallet).unwrap(); // ✅ Read wallet keypair
    let pubkey= payer.pubkey();
    println!("Payer address: {}", pubkey);

    // ✅ Use anchor-client to connect to Solana
    let client = Client::new_with_options(Cluster::Localnet, Rc::new(&payer), CommitmentConfig::processed());
    let program = client.program(Pubkey::from_str(program_id).unwrap()).unwrap();

    // ✅ Create an Album instance
    let contract_request = ContractRequest {
        album_id: "test-album-id".into(), // Random album ID
        price: 10.0, 
        credits: vec![
            Credits {
                artist_id: "test-artist-id".into(), // Random artist ID
                writing: 50.0,
                performance: 50.0,
            },
            Credits {
                artist_id: "test-artist-id-2".into(), // Random artist ID
                writing: 25.0,
                performance: 75.0,
            },
        ],
        // resale_allowed: true,
    };

    // ✅ Call the on-chain function (from lib.rs)
    let tx = program
        .request()
        .accounts(vec![AccountMeta::new_readonly(pubkey, false)])
        .args(MintAlbum { contract_request }) // ✅ Pass Album as a single argument
        .signer(&payer) // ✅ Sign the transaction
        .send()
        .expect("Failed to mint album");

    println!("Transaction Signature: {}", tx);
}