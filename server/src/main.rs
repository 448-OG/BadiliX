use common::{Poap, RandomBytes, Utils};
use ed25519_dalek::SecretKey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction,
};
use spl_token_2022::{extension::ExtensionType, state::Mint};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

mod poap_ops;
use poap_ops::*;

mod types;
use types::*;

mod ata;
use ata::*;

mod txs;
use txs::*;

mod program_utils;
use program_utils::*;

fn main() {
    let authority_keypair = Keypair::from_bytes(&[
        221, 138, 79, 165, 198, 65, 148, 172, 141, 152, 228, 17, 124, 124, 229, 80, 26, 128, 236,
        105, 94, 119, 134, 201, 5, 32, 90, 213, 9, 116, 172, 168, 93, 88, 48, 22, 130, 79, 230,
        210, 105, 156, 125, 206, 40, 13, 0, 220, 89, 187, 94, 220, 61, 135, 160, 193, 210, 247,
        221, 198, 221, 142, 56, 86,
    ])
    .unwrap();
    let mint_keypair = Keypair::from_bytes(&[
        132, 238, 70, 214, 8, 108, 24, 11, 165, 83, 223, 51, 79, 166, 230, 241, 193, 57, 12, 9, 6,
        217, 202, 148, 97, 121, 126, 223, 81, 129, 131, 84, 43, 171, 182, 89, 183, 235, 194, 242,
        182, 212, 26, 133, 155, 46, 178, 206, 18, 228, 33, 144, 220, 87, 106, 89, 59, 188, 227, 5,
        51, 70, 224, 145,
    ])
    .unwrap();
    println!("AUTHORITY: {:?}", authority_keypair.pubkey());
    println!("MINT: {:?}", mint_keypair.pubkey());

    // //let recipient_key = RandomBytes::<32>::gen();
    // let recipient_key_recover = [
    //     77u8, 118, 160, 107, 117, 195, 82, 218, 35, 236, 5, 231, 109, 19, 54, 83, 138, 177, 93, 96,
    //     157, 52, 147, 38, 207, 180, 8, 56, 19, 161, 87, 10,
    // ];

    // let recipient_phone = "254700000000";
    // let recipient_key = RandomBytes::<32>::from_bytes(recipient_key_recover);
    // let recipient = blake3::keyed_hash(&recipient_key.expose(), recipient_phone.as_bytes());

    // let encoded_key = Utils::encode_32byte_dashed(recipient_key.expose_borrowed());

    // let decoded = Utils::decode_dashed_bytes(&encoded_key);
    // assert_eq!(decoded.as_slice(), recipient_key.expose_borrowed());

    // let recipient_secret_key = SecretKey::from_bytes(recipient.as_bytes()).unwrap();
    // let recipient_public_key_temp: ed25519_dalek::PublicKey = (&recipient_secret_key).into();
    // let recipient_public_key = Pubkey::new_from_array(*recipient_public_key_temp.as_bytes());
    // let mut recipient_keypair_bytes = Vec::<u8>::new();
    // recipient_keypair_bytes.extend_from_slice(recipient_secret_key.as_bytes());
    // recipient_keypair_bytes.extend_from_slice(&recipient_public_key.to_bytes());
    // let recipient_keypair = Keypair::from_bytes(&recipient_keypair_bytes).unwrap();

    // println!("RECIPIENT: {:?}", recipient_keypair.pubkey());

    let localhost = "http://localhost:8899".to_string();
    let client = RpcClient::new(localhost);

    let mint_keys = MintKeys {
        mint: mint_keypair.pubkey(),
        authority: authority_keypair.pubkey(),
        update_authority: authority_keypair.pubkey(),
    };

    println!(
        "Airdrop for Mint Authority Reflected: {}",
        ProgramUtils::airdrop(&client, &authority_keypair.pubkey())
    );

    let poap_ixs = create_poap_mint(&client, &mint_keys);
    let create_poap_mint_sig = Tx::new(&authority_keypair)
        .add_instructions(poap_ixs)
        .add_signers(&[&authority_keypair, &mint_keypair])
        .send_tx(&client);

    println!("POAP CREATE SIG: {:?}", create_poap_mint_sig);
}
