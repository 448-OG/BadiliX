use solana_client::{rpc_client::RpcClient, rpc_config::RpcRequestAirdropConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
};

pub struct ProgramUtils;

impl ProgramUtils {
    pub fn airdrop(client: &RpcClient, recipient: &Pubkey) -> String {
        let mut airdrop_config = RpcRequestAirdropConfig::default();
        airdrop_config
            .commitment
            .replace(CommitmentConfig::confirmed());

        let current_balance = Self::get_balance(client, recipient);

        if current_balance > (1u64 * LAMPORTS_PER_SOL) {
            return Self::format_sol(current_balance);
        }

        client
            .request_airdrop_with_config(recipient, 1 * LAMPORTS_PER_SOL, airdrop_config)
            .unwrap();

        let new_balance = Self::get_balance(client, recipient);

        return Self::format_sol(new_balance);
    }

    pub fn get_balance(client: &RpcClient, recipient: &Pubkey) -> u64 {
        client
            .get_balance_with_commitment(recipient, CommitmentConfig::default())
            .unwrap()
            .value
    }

    pub fn format_sol(lamports: u64) -> String {
        let prepare_value = lamports as f64 / LAMPORTS_PER_SOL as f64;

        prepare_value.to_string() + "SOL"
    }
}
