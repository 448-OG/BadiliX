use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct Fx {
    buyer: Pubkey,
    amount: u64,
}

impl Fx {
    pub fn new(buyer: Pubkey, amount: u64) -> Self {
        Self { buyer, amount }
    }

    pub fn buyer(&self) -> Pubkey {
        self.buyer
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn exchange(&self, rate: u8, decimals: u64) -> u64 {
        (self.amount * (1 * decimals)) / (rate as u64)
    }
}
