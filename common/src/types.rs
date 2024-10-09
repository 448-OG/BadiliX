use std::sync::LazyLock;

use borsh::{BorshDeserialize, BorshSerialize};
use ed25519_dalek::SecretKey;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::{Config, RandomBytes, Utils};

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

#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct Ed25519KeypairBytes([u8; 64]);

impl Default for Ed25519KeypairBytes {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Default)]
pub struct RecipientInfo {
    phone_number: String,
    kdf_key: [u8; 32],
    keypair: Ed25519KeypairBytes,
}

impl RecipientInfo {
    pub fn new(recipient_phone: &str) -> Self {
        let mut info = Self::default();
        info.phone_number = recipient_phone.to_string();
        info.gen_kdf().set_kdf_keypair()
    }

    pub fn temp_kdf(&self, mint: Pubkey) -> blake3::Hash {
        blake3::Hasher::new_derive_key(&mint.to_string())
            .update(&self.kdf_key)
            .finalize()
    }

    pub fn temp_keypair(&self, mint: Pubkey) -> (blake3::Hash, Keypair) {
        let temp_kdf = self.temp_kdf(mint);
        let temp_keypair = Self::gen_kdf_keypair(temp_kdf.as_bytes());

        (temp_kdf, temp_keypair)
    }

    pub fn gen_kdf(mut self) -> Self {
        let recipient_key = Self::gen_rand();

        let encoded_key = Utils::encode_32byte_dashed(recipient_key.expose_borrowed());

        let decoded = Utils::decode_dashed_bytes(&encoded_key);
        assert_eq!(decoded.as_slice(), recipient_key.expose_borrowed());

        self.kdf_key = recipient_key.expose();

        self
    }

    fn gen_rand() -> RandomBytes<32> {
        RandomBytes::<32>::gen()
    }

    pub fn kdf_key_hash(&self) -> blake3::Hash {
        blake3::keyed_hash(&self.kdf_key, self.phone_number.as_bytes())
    }

    pub fn set_kdf_keypair(mut self) -> Self {
        self.keypair = Ed25519KeypairBytes(Self::gen_kdf_keypair(&self.kdf_key).to_bytes());

        self
    }

    pub fn gen_kdf_keypair(kdf_key_hash: &[u8; 32]) -> Keypair {
        let recipient_secret_key = SecretKey::from_bytes(kdf_key_hash).unwrap();
        let recipient_public_key_temp: ed25519_dalek::PublicKey = (&recipient_secret_key).into();
        let recipient_public_key = Pubkey::new_from_array(*recipient_public_key_temp.as_bytes());
        let mut recipient_keypair_bytes = Vec::<u8>::new();
        recipient_keypair_bytes.extend_from_slice(recipient_secret_key.as_bytes());
        recipient_keypair_bytes.extend_from_slice(&recipient_public_key.to_bytes());

        Keypair::from_bytes(&recipient_keypair_bytes).unwrap()
    }

    pub fn keypair(&self) -> Keypair {
        Keypair::from_bytes(&self.keypair.0).unwrap()
    }

    pub fn key(&self) -> RandomBytes<32> {
        RandomBytes::<32>::from_bytes(self.kdf_key)
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn set(&self) {
        let packed = borsh::to_vec(&self).unwrap();

        EVENTS_DB.insert(self.phone_number.clone(), packed).unwrap();
    }

    pub fn get(phone_number: &str) -> Option<Self> {
        EVENTS_DB
            .get(phone_number)
            .unwrap()
            .map(|user_exists| Self::try_from_slice(&user_exists).unwrap())
    }
}
