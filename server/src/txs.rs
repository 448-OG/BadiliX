use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

#[derive(Debug, Default)]
pub struct Tx<'a> {
    payer: Pubkey,
    instructions: Vec<Instruction>,
    signers: Vec<&'a Keypair>,
}

impl<'a> Tx<'a> {
    pub fn new(payer: &'a Keypair) -> Self {
        Self {
            payer: payer.pubkey(),
            signers: vec![payer],
            ..Default::default()
        }
    }

    pub fn add_signer(mut self, signer: &'a Keypair) -> Self {
        self.signers.push(signer);

        self
    }

    pub fn add_signers(mut self, signers: &[&'a Keypair]) -> Self {
        signers.iter().for_each(|signer| {
            self.signers.push(signer);
        });

        self
    }

    pub fn add_instruction(mut self, ix: Instruction) -> Self {
        self.instructions.push(ix);

        self
    }

    pub fn add_instructions(mut self, ixs: Vec<Instruction>) -> Self {
        ixs.into_iter()
            .for_each(|instruction| self.instructions.push(instruction));

        self
    }

    pub fn send_tx(&self, client: &RpcClient) -> Signature {
        let mut tx_config = RpcSendTransactionConfig::default();
        tx_config.skip_preflight = true;

        let recent_blockhash = client.get_latest_blockhash().unwrap();

        let tx = Transaction::new_signed_with_payer(
            self.instructions.as_slice(),
            Some(&self.payer),
            self.signers.as_slice(),
            recent_blockhash,
        );

        client
            .send_and_confirm_transaction_with_spinner_and_config(
                &tx,
                CommitmentConfig::confirmed(),
                tx_config,
            )
            .unwrap()
    }
}
