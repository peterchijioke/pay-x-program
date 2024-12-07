mod create_account;
mod deposit;
mod withdraw;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;




entrypoint!(initialize_process);

pub fn initialize_process(program_id:&Pubkey,accounts:&[AccountInfo],instruction_data:&[u8])->ProgramResult{
    


    match instruction_data[0] {
        0=>create_account::process(program_id,accounts),
        1=>deposit::process(program_id,accounts),
        2=>withdraw::process(program_id,accounts),
         _ => Err(ProgramError::InvalidInstructionData),
    }
}
