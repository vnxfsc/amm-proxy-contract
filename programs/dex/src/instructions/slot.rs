use solana_program::{
    clock::Clock, entrypoint::ProgramResult, program_error::ProgramError, sysvar::Sysvar,
};

pub const EXPIRED_SLOT_SELECTOR: &[u8; 8] = &[169, 134, 33, 62, 168, 2, 246, 176];

#[derive(Debug, Clone)]
pub enum MyError {
    SlotExpired,
}

impl From<MyError> for ProgramError {
    fn from(e: MyError) -> Self {
        ProgramError::Custom(e as u32) // 自定义错误码更清晰
    }
}
pub fn process_expired_slot(instruction_data: &[u8]) -> ProgramResult {
    let expiry_slot = u64::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let clock = Clock::get()?;

    if clock.slot > expiry_slot {
        return Err(MyError::SlotExpired.into());
    }

    Ok(())
}
