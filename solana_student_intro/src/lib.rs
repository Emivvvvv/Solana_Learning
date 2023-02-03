use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::AccountInfo,
};

pub mod instruction;
use instruction::{IntroInstruction};

entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = IntroInstruction::unpack(instruction_data)?;
    match instruction {
        IntroInstruction::AddStudent {name, student_id, message}
            => add_student(program_id, accounts, name, student_id, message)
    }
}

pub fn add_student(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    name: String,
    student_id: u8,
    message: String
) -> ProgramResult {
    msg!("Adding student message...");
    msg!("id - Student name: {}, {}", student_id, name);
    msg!("Student message: {}", message);
    Ok(())
}