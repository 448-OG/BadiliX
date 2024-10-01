use tai64::Tai64N;

use crate::{Blake3HashBytes, Ed25519PublicKeyBytes, TaiTimestamp};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Poap<'a> {
    recipient: Blake3HashBytes,
    timestamp: TaiTimestamp,
    issuer: Ed25519PublicKeyBytes,
    data: &'a [u8],
}

impl<'a> Poap<'a> {
    pub fn new(issuer: Ed25519PublicKeyBytes) -> Self {
        Self {
            issuer,
            ..Default::default()
        }
    }

    pub fn add_recipient(mut self, wallet: Blake3HashBytes) -> Self {
        self.recipient = wallet;

        self
    }

    pub fn add_timestamp(mut self, timestamp: TaiTimestamp) -> Self {
        self.timestamp = timestamp;

        self
    }

    pub fn add_data(mut self, data: &'a impl AsRef<[u8]>) -> Self {
        self.data = data.as_ref();

        self
    }

    fn identifier(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.issuer);
        hasher.update(&self.recipient);

        let timestamp = Tai64N::now().to_bytes();
        hasher.update(&timestamp);

        hasher.update(&self.data);

        hasher.finalize()
    }

    pub fn validate(&self, hash_bytes: Blake3HashBytes) -> bool {
        let hash: blake3::Hash = hash_bytes.into();
        self.identifier() == hash
    }

    pub fn recipient(&self) -> Blake3HashBytes {
        self.recipient
    }

    pub fn timestamp(&self) -> TaiTimestamp {
        self.timestamp
    }

    pub fn issuer(&self) -> Ed25519PublicKeyBytes {
        self.issuer
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut packed = Vec::<u8>::new();

        packed.extend_from_slice(&self.issuer);
        packed.extend_from_slice(&self.recipient);
        packed.extend_from_slice(&self.timestamp);
        packed.extend_from_slice(&self.data);

        packed
    }

    pub fn unpack(bytes: &'a [u8]) -> Self {
        let issuer = bytes[0..32].try_into().unwrap();
        let recipient = bytes[32..64].try_into().unwrap();
        let timestamp = bytes[64..76].try_into().unwrap();
        let data = &bytes[76..];

        Self {
            recipient,
            timestamp,
            issuer,
            data,
        }
    }
}
