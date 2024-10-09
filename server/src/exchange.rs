
use common::{ProgramUtils, Tx, AUTHORITY, EXCHANGE_RATE, USD_MINT};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token_2022::instruction::mint_to;

pub(crate) fn exchange_usd(fx: Fx, recipient_keypair: &Keypair, client: &RpcClient) -> Signature {
    println!(
        "Init with SOL: {:?}",
        ProgramUtils::init_with_sol(&recipient_keypair.pubkey(), client)
    );

    let usd_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &recipient_keypair.pubkey(),
        &USD_MINT.pubkey(),
        &spl_token_2022::id(),
    );

    println!("USD ATA:{}", usd_ata);

    let create_usd_ata_ix = create_associated_token_account(
        &recipient_keypair.pubkey(),
        &recipient_keypair.pubkey(),
        &USD_MINT.pubkey(),
        &spl_token_2022::id(),
    );

    let exchange_usd_amount = fx.exchange(EXCHANGE_RATE, 6);

    let mint_usd_ix = mint_to(
        &spl_token_2022::id(),
        &USD_MINT.pubkey(),
        &usd_ata,
        &AUTHORITY.pubkey(),
        &[&AUTHORITY.pubkey()],
        exchange_usd_amount,
    )
    .unwrap();

    Tx::new(&AUTHORITY)
        .add_signer(&recipient_keypair)
        .add_instructions(vec![create_usd_ata_ix, mint_usd_ix])
        .send_tx(client)
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
