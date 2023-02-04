use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked
};
use std::convert::TryInto;
pub mod instruction;
pub mod state;
use instruction::IntroInstruction;
use state::StudentInfo;
use borsh::BorshSerialize;


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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    student_id: u32,
    message: String
) -> ProgramResult {
    msg!("Adding student message...");
    msg!("id - Student name: {}, {}", student_id, name);
    msg!("Student message: {}", message);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(),], program_id);

    let account_len: usize = 1 + (4+name.len()) + 4 + (4+message.len());

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            user_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(), user_account.clone(), system_program.clone()],
        &[&[initializer.key.as_ref(),&[bump_seed]]],
    )?;

    msg!("PDA created: {}", pda);
    msg!("unpacking state account");
    let mut account_data = try_from_slice_unchecked::<StudentInfo>(&user_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    account_data.name = name;
    account_data.msg = message;
    account_data.student_id = student_id;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}