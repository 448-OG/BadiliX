use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    api_key: String,
}

impl Config {
    pub fn read() -> Self {
        let mut file = std::fs::File::open("../at_api.key.toml").unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        toml::from_str::<Self>(&contents).unwrap()
    }

    pub fn api_key(&self) -> &String {
        &self.api_key
    }
}
