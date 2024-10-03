use crate::MintKeys;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};

pub fn ata(mintkeys: &MintKeys, recipient: Pubkey) -> (Pubkey, Instruction) {
    let ata = get_associated_token_address_with_program_id(
        &recipient,
        &mintkeys.mint,
        &spl_token_2022::id(),
    );

    let mint_authority_ata_instr = create_associated_token_account(
        &mintkeys.authority,
        &mintkeys.authority,
        &mintkeys.mint,
        &spl_token_2022::id(),
    );

    (ata, mint_authority_ata_instr)
}
