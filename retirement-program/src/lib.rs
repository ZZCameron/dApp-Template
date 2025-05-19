use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Ensure at least one account (the user's wallet) is provided
    if accounts.is_empty() {
        return Err(solana_program::program_error::ProgramError::NotEnoughAccountKeys);
    }

    let user_account = &accounts[0];
    if !user_account.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    // No-op: Just validate the signature and return success
    Ok(())
}