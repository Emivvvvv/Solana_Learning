use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError};

pub enum IntroInstruction {
    AddStudent {
        name: String,
        student_id: u8,
        message: String,
    }
}

#[derive(BorshDeserialize)]
struct StudentInfoPayload {
    name: String,
    student_id: u8,
    message: String,
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentInfoPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::AddStudent {
                name: payload.name,
                student_id: payload.student_id,
                message: payload.message },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}