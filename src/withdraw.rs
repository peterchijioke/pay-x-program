use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;

    let amount_to_withdraw = **user_account.lamports.borrow();

    if amount_to_withdraw == 0 {
        return Err(ProgramError::InsufficientFunds);
    }

    let fee = 100_000; 
    let amount_after_fee = amount_to_withdraw - fee;

    **user_account.lamports.borrow_mut() -= amount_after_fee;
    **destination_account.lamports.borrow_mut() += amount_after_fee;

    Ok(())
}