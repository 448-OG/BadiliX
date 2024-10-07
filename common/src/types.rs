use std::sync::LazyLock;

use solana_program::pubkey::Pubkey;

use crate::Config;

pub const EVENTS_DB: LazyLock<sled::Db> = LazyLock::new(|| sled::open("../POAP-MINTS").unwrap());
pub const APP_CONFIG: LazyLock<Config> = LazyLock::new(|| Config::read());

pub type Blake3HashBytes = [u8; 32];
pub type TaiTimestamp = [u8; 12];
pub type Ed25519PublicKeyBytes = [u8; 32];

#[derive(Debug)]
pub struct MintKeys {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug)]
pub struct MintMetadata {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub additional_metadata: (String, String),
}
