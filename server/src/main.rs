use std::sync::LazyLock;

use common::{AUTHORITY, USD_MINT};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::Signer;

mod ussd;
use ussd::*;

mod poap;
use poap::*;

mod messaging;
use messaging::*;

mod exchange;
use exchange::*;

pub(crate) const CLIENT: LazyLock<RpcClient> = LazyLock::new(|| {
    let devnet = "https://api.devnet.solana.com".to_string();
    RpcClient::new(devnet)
});

fn main() {
    println!("AUTHORITY: {}", AUTHORITY.pubkey());
    println!("USD MINT: {}", USD_MINT.pubkey());
    println!("AUTHORITY: {}", AUTHORITY.to_base58_string());
    println!("USD MINT: {}", USD_MINT.to_base58_string());

    // println!(
    //     "Airdrop for Mint Authority Reflected: {}",
    //     ProgramUtils::airdrop(&client, &authority.pubkey())
    // ); FIXME

    start_server();
}
