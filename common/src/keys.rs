use std::sync::LazyLock;

use solana_sdk::signature::Keypair;

use crate::APP_CONFIG;

pub const AUTHORITY: LazyLock<Keypair> = LazyLock::new(|| {
    let authority_bytes = bs58::decode(APP_CONFIG.authority()).into_vec().unwrap();
    Keypair::from_bytes(&authority_bytes).unwrap()
});

pub const USD_MINT: LazyLock<Keypair> = LazyLock::new(|| {
    let mint_bytes = bs58::decode(APP_CONFIG.mint()).into_vec().unwrap();
    Keypair::from_bytes(&mint_bytes).unwrap()
});

pub const EXCHANGE_RATE: u8 = 129;
