
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::program_error::ProgramError;
pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let source_account = next_account_info(accounts_iter)?;

    if **source_account.lamports.borrow() < 1 {
        return Err(ProgramError::InsufficientFunds);
    }

    let fee = 100_000; 
    let amount_to_transfer = **source_account.lamports.borrow() - fee;
    **user_account.lamports.borrow_mut() += amount_to_transfer;
    **source_account.lamports.borrow_mut() -= amount_to_transfer;
    Ok(())
}