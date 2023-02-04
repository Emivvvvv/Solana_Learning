use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize)]
pub struct StudentInfo {
    pub is_initialized: bool,
    pub name: String,
    pub student_id: u32,
    pub msg: String,
}