use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};


entrypoint!(hello_solana);


fn hello_solana(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");
    msg!("This program's Program ID: {}", &program_id);
    msg!("This program is created by Emivvvvv with a hurry to catch Solana GRIZZLYTHON!!!");
    Ok(())
}