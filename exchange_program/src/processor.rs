use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use spl_token_2022::{
    extension::StateWithExtensions,
    state::Mint,
};

pub fn exchange(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info = &mut accounts.iter();
    let quote = next_account_info(account_info)?;

    let quote_data = &quote.data.borrow();

    let quote_info = StateWithExtensions::<Mint>::unpack(quote_data)?;
    msg!("QUOTE DECIMALS: {:?}", &quote_info.base.decimals);

    Ok(())
}
