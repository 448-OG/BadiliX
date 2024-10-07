use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};

mod poap_ops;
use poap_ops::*;

fn main() {
    let authority = Keypair::from_bytes(&[
        221, 138, 79, 165, 198, 65, 148, 172, 141, 152, 228, 17, 124, 124, 229, 80, 26, 128, 236,
        105, 94, 119, 134, 201, 5, 32, 90, 213, 9, 116, 172, 168, 93, 88, 48, 22, 130, 79, 230,
        210, 105, 156, 125, 206, 40, 13, 0, 220, 89, 187, 94, 220, 61, 135, 160, 193, 210, 247,
        221, 198, 221, 142, 56, 86,
    ])
    .unwrap();
    let mint = Keypair::from_bytes(&[
        132, 238, 70, 214, 8, 108, 24, 11, 165, 83, 223, 51, 79, 166, 230, 241, 193, 57, 12, 9, 6,
        217, 202, 148, 97, 121, 126, 223, 81, 129, 131, 84, 43, 171, 182, 89, 183, 235, 194, 242,
        182, 212, 26, 133, 155, 46, 178, 206, 18, 228, 33, 144, 220, 87, 106, 89, 59, 188, 227, 5,
        51, 70, 224, 145,
    ])
    .unwrap();
    println!("AUTHORITY: {:?}", authority.pubkey());
    println!("MINT: {:?}", mint.pubkey());

    let localhost = "http://localhost:8899".to_string();
    let client = RpcClient::new(localhost);

    create_poap_mint(&authority, &mint, &client);
}
