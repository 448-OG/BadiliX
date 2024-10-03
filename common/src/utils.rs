use data_encoding::HEXUPPER;

pub struct Utils;

impl Utils {
    pub fn encode_32byte_dashed(bytes: &[u8; 32]) -> String {
        Self::encode_dashed(&HEXUPPER.encode(bytes))
    }

    pub fn encode_dashed(value: &str) -> String {
        let stringified_hex = value.to_string();
        let mut dashed_encoded = String::new();

        stringified_hex
            .chars()
            .enumerate()
            .for_each(|(index, char)| {
                if index > 0 && index % 4 == 0 {
                    dashed_encoded.push('-');
                }
                dashed_encoded.push(char);
            });

        dashed_encoded
    }

    pub fn decode_dashed(hash_dashed: &str) -> String {
        hash_dashed.trim().replace("-", "")
    }

    pub fn decode_dashed_blake3(hash_dashed: &str) -> blake3::Hash {
        let decoded = Self::decode_dashed(hash_dashed);
        blake3::Hash::from_hex(&decoded).unwrap()
    }

    pub fn decode_dashed_bytes(hash_dashed: &str) -> Vec<u8> {
        let decoded = Self::decode_dashed(hash_dashed);
        HEXUPPER.decode(decoded.as_bytes()).unwrap()
    }
}
