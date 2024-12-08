use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::program_error::ProgramError;
use solana_program::system_instruction;
use solana_program::sysvar::{rent::Rent, Sysvar};
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;

    if user_account.data_is_empty() {
        let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
        let rent_exempt = rent.is_exempt(user_account.lamports(), user_account.data_len());

        if !rent_exempt {
            return Err(ProgramError::InsufficientFunds);
        }

        let system_program = next_account_info(accounts_iter)?;
        let create_account_ix = system_instruction::create_account(
            &system_program.key,
            &user_account.key,
            1_000_000_000, 
            0, 
            program_id,
        );

        solana_program::program::invoke(
            &create_account_ix,
            &[
                system_program.clone(),
                user_account.clone(),
            ],
        )?;

        Ok(())
    } else {
        Err(ProgramError::AccountAlreadyInitialized)
    }
}